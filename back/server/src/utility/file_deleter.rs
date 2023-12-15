use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver as Receiver;
use tokio::sync::mpsc::UnboundedSender as Sender;
use std::thread::JoinHandle;

use crate::utility::img::ImgDelInfo;

const DELETE_LOOP_SECS: u64 = 30; // 300;
const DELETE_LOOP_DUR: Duration = Duration::from_secs(DELETE_LOOP_SECS);

//TODO: wrap `State` into struct 
pub type State = Arc<Mutex<FileDelState>>;
type Act = SingleDelAction;
type Acts = Vec<Act>;

fn state_lock_mut(state: &State) -> MutexGuard<'_, FileDelState> {
    let state = state.lock();
    let mut state = match state {
        Ok(state) => { state },
        Err(state) => {
            println!("[WARN] file state poisoned. it's ok?");
            state.into_inner()
        }
    };
    state
}

pub mod global {
    use std::sync::OnceLock;
    use super::{State, Act};
    use super::ImgDelInfo;

    static GLOBAL_STATE: OnceLock<State> = OnceLock::new();

    pub fn is_global_inited() -> bool {
        GLOBAL_STATE.get().is_some()
    }
    
    /// # Err
    /// * if `is_global_inited()`: global state already was inited
    pub fn init_global(state: State) -> Result<(), ()> {
        GLOBAL_STATE.set(state).map_err(|_|())
    }
    
    /// # Err
    /// * if `!is_global_inited()`: global state wasn't inited
    pub fn add_del_pics_act(
        board_url: String, 
        pics_info: Vec<ImgDelInfo>,
    ) -> Result<(), ()> {
        match GLOBAL_STATE.get() {
            Some(state) => {
                let mut state = super::state_lock_mut(state);
                state.add_act(Act::DelPics { board_url, pics_info });
                Ok(())
            }
            _ => Err(())
        }
    }
}

enum SingleDelAction {
    DelPics {
        board_url: String, 
        pics_info: Vec<ImgDelInfo>,
    },
}

impl SingleDelAction {
    fn del(self) {
        match self {
            Self::DelPics{board_url, pics_info} => {
                for pic_info in pics_info {
                    //TODO: DEL PIC 
                }
            }
        }
    }
}

pub struct FileDelState {
    pic_path: String,
    acts: Acts,
}

impl FileDelState {
    fn new<S: Into<String>>(pic_path: S) -> Self { 
        Self {
            pic_path: pic_path.into(),
            acts: Vec::with_capacity(1 << 6),
        }
    }

    fn add_act(&mut self, act: Act) {
        self.acts.push(act)
    }

    fn take_cur_acts(take_to: &mut Acts, take_from: &mut Self) {
        take_to.append(&mut take_from.acts)
    }
}

pub struct FileDeleter {
    state: State,
    acts_buf: Acts,

    cmd_end_rx: Receiver<()>,
    cmd_done_sx: Sender<()>,
}

impl FileDeleter {
    pub fn new<S: Into<String>>(pic_path: S) -> FileDeleterCtrl {
        let (cmd_end_sx, cmd_end_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        let (cmd_done_sx, cmd_done_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        
        let state = Arc::new(Mutex::new(FileDelState::new(pic_path)));

        let x = Self {
            state: state.clone(),
            acts_buf: Acts::with_capacity(1 << 6),
            cmd_end_rx,
            cmd_done_sx,
        };

        let thr_loop_handler = x.del_loop_start();
        FileDeleterCtrl::new(state, cmd_end_sx, cmd_done_rx, thr_loop_handler)
    }

    fn del_loop_start(self) -> JoinHandle<()> {
        std::thread::spawn(move||{
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move { self.del_loop().await })
        })
    }

    async fn del_loop(mut self) {
        'del_loop: loop {
            tokio::select! {
                () = tokio::time::sleep(DELETE_LOOP_DUR) => {
                    self.del_step();
                }
                Some(_) = self.cmd_end_rx.recv() => {
                    self.del_step();
                    break 'del_loop
                }
            }
        }
    }

    fn del_step(&mut self) {
        '_block_state: {
            let mut state = state_lock_mut(&self.state);
            FileDelState::take_cur_acts(&mut self.acts_buf, &mut state);
        }

        for act in self.acts_buf.drain(..) {
            act.del();
        }
    }
}

pub struct FileDeleterCtrl {
    state: State,

    cmd_end_sx: Sender<()>,
    cmd_done_rx: Receiver<()>,

    thr_loop_handler: JoinHandle<()>, 
}

impl FileDeleterCtrl {
    fn new(
        state: State,
        cmd_end_sx: Sender<()>,
        cmd_done_rx: Receiver<()>,
        thr_loop_handler: JoinHandle<()>, 
    ) -> Self {
        Self {
            state,
            cmd_end_sx,
            cmd_done_rx,
            thr_loop_handler,
        }
    }

    /// # panic
    /// * if `global::is_global_inited()` global state already was inited
    pub fn init_global_state(&self) {
        global::init_global(self.state()).unwrap()
    }

    pub fn state(&self) -> State {
        Arc::clone(&self.state)
    }

    pub fn close(self) -> anyhow::Result<()> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async move { self.close_inner().await })?;

        Ok(())
    }
    async fn close_inner(mut self) -> anyhow::Result<()> {
        self.cmd_end_sx.send(())?;
        self.cmd_done_rx.recv().await;
        
        anyhow::ensure!(
            self.thr_loop_handler.join().is_ok(), 
            "... close cmd done but loop thread paniced ://"
        );

        Ok(())
    }
}
