use crate::utility::img::ImgDelInfo;
use super::help::*;
use super::LoopDur;

const DELETE_LOOP_SECS: u64 = 300;
pub const DELETE_LOOP_DUR: Duration = Duration::from_secs(DELETE_LOOP_SECS);

//TODO:MAYBE: wrap `State` into struct 
pub type SharedActs = Arc<Mutex<FileDelActs>>;
type Act = SingleDelAction;
type Acts = Vec<Act>;

fn state_lock(state: &SharedActs) -> MutexGuard<'_, FileDelActs> {
    let state = state.lock();
    match state {
        Ok(state) => { state }
        Err(state) => {
            println!("[WARN] file state poisoned. it's ok?");
            state.into_inner()
        }
    }
}

pub mod global {
    use std::sync::OnceLock;
    use super::{SharedActs, Act};
    use super::ImgDelInfo;

    static GLOBAL_STATE: OnceLock<SharedActs> = OnceLock::new();

    #[allow(unused)]
    pub fn is_global_inited() -> bool {
        GLOBAL_STATE.get().is_some()
    }
    
    /// # Err
    /// * if `is_global_inited()`: global state already was inited
    pub fn init_global(state: SharedActs) -> Result<(), ()> {
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
                let mut state = super::state_lock(state);
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
    fn del(self, pic_path_parent: &str) {
        match self {
            Self::DelPics{board_url, pics_info} => {
                let path = format!("{pic_path_parent}/{board_url}");
                pics_info.into_iter().for_each(|x|x.del(&path));
            }
        }
    }
}

pub struct FileDelActs {
    acts: Acts,
}

impl FileDelActs {
    fn new() -> Self { 
        Self {
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

pub struct FileDelState {
    pic_path_parent: String,
    acts: SharedActs,
    acts_buf: Acts,
}

impl FileDelState {
    pub fn new<S: Into<String>>(pic_path_parent: S) -> Self {
        Self {
            pic_path_parent: pic_path_parent.into(),
            acts: Arc::new(Mutex::new(FileDelActs::new())),
            acts_buf: Acts::with_capacity(1 << 6),
        }
    }
}
impl LoopActor for FileDelState {
    fn act_step(&mut self) {
        '_block_state: {
            let mut acts = state_lock(&self.acts);
            FileDelActs::take_cur_acts(&mut self.acts_buf, &mut acts);
        }

        for act in self.acts_buf.drain(..) {
            act.del(&self.pic_path_parent);
        }
    }
    
    fn close_step(&mut self) {
        self.act_step()
    }
    
    /// # panic
    /// * if `global::is_global_inited()` global state already was inited
    fn init(&self) {
        global::init_global(Arc::clone(&self.acts)).unwrap()
    } 
}

impl LoopDur for FileDelState {
    fn config_loop_dur() -> Duration {
        let secs = crate::config::Config::loops().auto_del_dt_sec;
        Duration::from_secs(secs)
    }
}
