use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver as Receiver;
use tokio::sync::mpsc::UnboundedSender as Sender;
use std::thread::JoinHandle;


const DELETE_LOOP_SEC: u64 = 30;
const DELETE_LOOP_DUR: Duration = Duration::from_secs(DELETE_LOOP_SEC);

pub type State = Arc<Mutex<FileDelState>>;
type Acts = Vec<SingleDelAction>;

enum SingleDelAction {
    DelPics(Vec<()>),
}

impl SingleDelAction {
    fn del(self) {
        match self {
            Self::DelPics(pics_info) => {
                for pic_info in pics_info {
                    //TODO: DEL PIC 
                }
            }
        }
    }
}

pub struct FileDelState {
    acts: Acts,
}

impl FileDelState {
    fn new() -> Self { 
        Self {
            acts: Vec::with_capacity(1 << 6),
        }
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
    pub fn new() -> FileDeleterCtrl {
        let (cmd_end_sx, cmd_end_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        let (cmd_done_sx, cmd_done_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        
        let state = Arc::new(Mutex::new(FileDelState::new()));

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
            let state = self.state.lock();
            let mut state = match state {
                Ok(state) => { state },
                Err(state) => {
                    println!("[WARN] file state poisoned. it's ok?");
                    state.into_inner()
                }
            };
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
