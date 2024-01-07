use std::collections::{HashMap, HashSet};
use anyhow::bail;
pub use serde::Serialize;
use crate::post::{Post, PostN};
use crate::thread::{Thread, ThreadOpN};
use crate::thread_usage_rate::ThreadUsageRate;

crate::define_id!(BoardId);

// TODO: wrap some fields in ~`Arc<RwLock<...>>` for non blocking operations

// TODO: maybe: arrayvec::ArrayString<16> for url ?
#[derive(Serialize, Debug, Clone)]
pub struct PopBoardInfo {
    pub url: String,
    pub name: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct PopBoardsTagInfo {
    pub tag: String,
    pub boards: Vec<PopBoardInfo>,
}


#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
pub struct Board {
    pub url: String,
    pub name: String,
    pub descr: String,
    pub post_qty: PostN,
    pub thr_limit: usize,

    // TODO:MAYBE: SpeedPost
    thrs: HashMap<ThreadOpN, Thread>,
    thrs_usage: ThreadUsageRate, 
}

impl Board {
    pub fn next_post_n(&mut self) -> PostN {
        self.post_qty += 1;
        self.post_qty
    }

    #[allow(unused)]
    /// * change max thread qty and if need delete mostly unbumped of threads
    pub fn set_thr_limit(&mut self, new_thr_limit: usize) {
        if new_thr_limit < self.thrs_qty() {
            //TODO:SPEED UP: fn `remove_min_n_bump_thr` that remove n thrs at once
            let need_n_remove = self.thrs_qty() - new_thr_limit;
            self.remove_min_n_bump_thr(need_n_remove);
        }
        self.thr_limit = new_thr_limit;
    }

    pub fn new(url: String, name: String, descr: String, post_qty: PostN, max_thr_qty: usize) -> Self {
        // const STD_THR_LIMIT: usize = 50;
        Self {
            url,
            name,
            descr,
            post_qty,
            thr_limit: max_thr_qty, // STD_THR_LIMIT,

            thrs: HashMap::new(),
            thrs_usage: ThreadUsageRate::new(),
        }
    }

    pub fn top_k_thrs_by_usage(&self, k: usize, except_n: &HashSet<u64>) -> Vec<&Thread> {
        //TODO:MAYBE:speed up
        //TODO:slow
        assert_ne!(k, 0);
        let mut ret = Vec::with_capacity(k + 1);

        let mut top_i = 0;
        loop {
            let Some(n) = self.thrs_usage.top_n(top_i) else { break };
            top_i += 1;
            if except_n.contains(&n.into()) { continue; }
            ret.push(self.thrs.get(&n).unwrap());
            if ret.len() == k { break }
        }
        return ret
    }

    pub fn top_thr_by_usage_n(&self, n: usize) -> Option<&Thread> {
        self.thrs_usage.top_n(n).map(|id|self.thrs.get(&id).unwrap())
    }

    pub fn new_thr_preproced(&mut self, header: String, mut op_post: Post, infinity: bool) -> u64 {
        op_post.upd_n(self.next_post_n());
        let thr_op_n = ThreadOpN::from(op_post.n());
        
        // delete least unbumped if need
        if self.thr_limit <= self.thrs.len() {
            self.remove_min_bump_thr()
        }
        
        assert!(self.thrs.insert(thr_op_n, Thread::new_preproced(header, op_post, infinity)).is_none());
        self.thrs_usage.add_new(thr_op_n);
        thr_op_n.into()
    }

    #[allow(unused)]
    pub fn new_thr(&mut self, header: Option<String>, op_post: Post, infinity: bool) -> u64 {
        let header = Thread::preproc_header(header, &op_post);
        self.new_thr_preproced(header, op_post, infinity)
    }

    pub fn remove_thr(&mut self, op_n: ThreadOpN) {
        if let Some(thr) = self.thrs.remove(&op_n) {
            thr.del(&self.url);
        }
        self.thrs_usage.remove_by_thr_n(op_n);
        // TODO:remove pic folder
    }

    fn remove_min_bump_thr(&mut self) {
        let mut iter = self.thr_iter();
        if let Some(thr) = iter.next() {
            let mut candidate_t = thr.bump_time();
            let mut candidate_n = thr.op_n();
            for thr in iter {
                let t = thr.bump_time();
                if t < candidate_t {
                    candidate_t = t;
                    candidate_n = thr.op_n();
                }
            }
            self.remove_thr(candidate_n)
        }
    }

    fn remove_min_n_bump_thr(&mut self, n_remove: usize) {
        //TODO:SPEED UP: we can do it for 1 iteration (use vec of candidate)
        for _ in 0..n_remove {
            self.remove_min_bump_thr();
        }
    }
    
    pub fn thr(&self, op_post_n: u64) -> Option<&Thread> {
        self.thrs.get(&ThreadOpN::from(op_post_n))
    }

    pub fn thr_mut(&mut self, op_post_n: u64) -> Option<&mut Thread> {
        self.thrs.get_mut(&ThreadOpN::from(op_post_n))
    }

    pub fn add_post(&mut self, op_post_n: u64, mut post: Post) -> Option<PostN> /* TODO: ret Result */ {
        let post_n = post.upd_n(self.next_post_n());

        if let Some(thr) = self.thr_mut(op_post_n) {
            thr.add_post(post);
            if !thr.is_bump_limit_reached() {
                let post_rate = thr.last_post_rate();
                let thr_op_n = thr.op_n();
                self.thrs_usage.upd_rates(thr_op_n, post_rate)
            }
            Some(post_n)
        } else {
            None
        }
    }

    pub fn thr_iter(&self) -> impl Iterator<Item = &Thread> {
        self.thrs.values()
    }

    pub fn is_thr_exist(&self, op_post_n: u64) -> bool {
        self.thrs.contains_key(&ThreadOpN::from(op_post_n))
    }

    pub fn thrs_rate(&self) -> Vec<u64> {
        self.thrs_usage.thrs_rate()
    }

    pub fn thrs_qty(&self) -> usize {
        self.thrs.len()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct BoardTag {
    pub tag: String,
}

#[ref_struct::ref_struct(
    ignore(boards),
    clone(next_board_id, board_qty, pop_board_qty, pic_n),
    derive(serde::Serialize, serde::Deserialize),
    ignore_struct()
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenBoards {
    boards: HashMap<BoardId, Board>,
    board_tags: HashMap<Option<BoardTag>, Vec<BoardId>>,
    board_urls: HashMap<String, BoardId>,
    next_board_id: BoardId,    
    board_qty: u32,
    pop_board_qty: u32,
    pic_n: u64,
    pic_path: String,
}

impl OpenBoards {
    pub fn new(dist_path: &str) -> Self {
        let pic_path = format!("{dist_path}/imgs/pp"); // pp stands for post pics 
        let _ = std::fs::create_dir(&pic_path);

        Self {
            boards: HashMap::new(),
            board_tags: HashMap::new(),
            board_urls: HashMap::new(),
            next_board_id: BoardId::first(),
            board_qty: 0,
            pop_board_qty: 0,
            pic_n: 0,
            pic_path,
        }
    }

    pub fn pic_path_parent(&self) -> &str {
        &self.pic_path
    }
    pub fn pic_path_unchecked(&self, board_url: &str) -> String {
        format!("{}/{}", self.pic_path.clone(), board_url)
    }
    #[allow(unused)]
    pub(in crate)
    fn pic_path(&self, board_id: BoardId) -> Option<String> {
        self.boards.get(&board_id)
            .map(|board|self.pic_path_unchecked(&board.url))
    }

    pub fn use_n_pic(&mut self, n: u64) -> std::ops::Range<u64> {
        let from = self.pic_n;
        let to = self.pic_n + n;
        self.pic_n += n;
        from..to
    }

    /// # WARNING
    /// use only for init
    pub(in super) fn x_post_qty(&self) -> u64 {
        self.boards.iter().fold(0, |ret, (_, b)|ret + b.post_qty)
    }

    pub fn open_boards_qty(&self) -> u32 {
        self.board_qty
    }
    pub fn pop_boards_qty(&self) -> u32 {
        self.pop_board_qty
    }

    /// # WARNING
    /// memorize result value! (until there no changes in `pop_boards_qty`)
    pub fn popular_boards(&self) -> Vec<PopBoardsTagInfo> {
        let mut ret = Vec::with_capacity(self.pop_board_qty as usize);
        for (tag, boards) in self.board_tags.iter() {
            if let Some(tag) = tag { 
                let boards = boards.iter().map(|id|self.boards.get(id).map(|x|PopBoardInfo{
                    url: x.url.clone(), 
                    name: x.name.clone(),
                }).unwrap()).collect();

                ret.push(PopBoardsTagInfo {
                    tag: tag.tag.clone(),
                    boards,
                })
            }
        }
        ret
    }

    // TODO: return Result<_, ..>
    #[must_use]
    /// # Return value
    /// * `true` => sucessfully added
    /// * `false` => url already used for other board 
    pub fn add_board(&mut self, board: Board, tag: Option<BoardTag>) -> anyhow::Result<()>
    {
        let max_url_len = crate::config::config().imageboard.max_board_url_len;
        if self.board_urls.contains_key(&board.url) { bail!("board url already used!") }
        if board.url.len() > max_url_len { bail!("too long board url :|") }

        self.board_qty += 1;
        if tag.is_some() { self.pop_board_qty += 1; }

        let id = self.next_board_id.inc();
        self.board_urls.insert(board.url.clone(), id);

        match self.board_tags.get_mut(&tag) {
            Some(boards) => { boards.push(id); }
            None => { self.board_tags.insert(tag, vec![id]); }
        }

        if !crate::utility::general::create_dir(format!("{}/{}", self.pic_path, board.url)) {
            bail!("OS error: cannot create dir for board")
        }

        self.boards.insert(id, board);
        Ok(())
    }

    pub fn is_board_exist(&self, board_url: &str) -> bool {
        self.board_urls.contains_key(board_url)
    }

    #[allow(unused)]
    pub(in crate)
    fn board_id(&self, board_url: &str) -> Option<BoardId> {
        self.board_urls.get(board_url).cloned()
    }

    pub fn board_name(&self, board_url: &str) -> Option<&str> {
        self.board(board_url).map(|b|b.name.as_str())
    }

    pub fn board(&self, board_url: &str) -> Option<&Board> {
        self.board_urls.get(board_url).map(|id|self.boards.get(id).unwrap())
    }
    pub(in crate)
    fn board_and_id(&self, board_url: &str) -> Option<(BoardId, &Board)> {
        self.board_id(board_url).map(|id|(id, self.boards.get(&id).unwrap()))
    }
    
    pub fn board_mut(&mut self, board_url: &str) -> Option<&mut Board> {
        self.board_urls.get(board_url).map(|id|self.boards.get_mut(id).unwrap())
    }
}

impl crate::config::ConfigCtor for OpenBoards {
    fn config_new(_: ()) -> Self {
        let vue_dist_path = crate::config::Config::vue_dist_path();
        Self::new(vue_dist_path)
    }
}

pub mod save_load {
    use std::io::{Read, Write};

    use crate::utility::save_load::*;
    use crate::utility::MutCell;
    use super::*;

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // TODO: if `OpenBoards` will be splited into diff RefTypes(`Arc`/`Mutex`/etc)
    //     | then we must block it all before save action (data consistency)
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    type Args<'x> = OpenBoardsArgs<'x>;

    pub enum OpenBoardsArgs<'x> {
        SingleFile(MutCell<'x, FileBufArgs>),
        BoardSplited{
            general_args: MutCell<'x, FileBufArgs>,
            save_path: MutCell<'x, String>,
        },
    }
    impl<'x> OpenBoardsArgs<'x> {
        fn board_path(save_path: &str, id: BoardId) -> String {
            format!("{save_path}/b{}.save", id.0)
        }
    } 

    impl<'x> Save<Args<'x>> for OpenBoards {
        fn save(&self, save_args: &mut Args) -> anyhow::Result<()> {
            match save_args {
                Args::SingleFile(save_args) => {
                    save_args.serialize_and_write(self)?;
                }
                Args::BoardSplited { general_args, save_path } => {
                    let all_except_boards = super::RefOpenBoards::new(self);
                    general_args.serialize_and_write(all_except_boards)?;

                    for (id, board) in &self.boards {
                        let path = OpenBoardsArgs::board_path(save_path, *id);
                        let mut board_file = std::fs::File::create(path)?;

                        general_args.serialize(board)?;
                        board_file.write_all(general_args.buf.as_slice())?;
                        general_args.clear();
                    }
                }
            }
            Ok(())
        }
    }

    impl<'x> Load<Args<'x>> for OpenBoards {
        fn load(load_args: &mut Args) -> anyhow::Result<Self> {
            match load_args {
                Args::SingleFile(load_args) => {
                    return Ok(load_args.read_and_deserialize()?)
                }
                Args::BoardSplited { general_args, save_path } => {
                    let all_except_boards: RefOpenBoards = general_args.read_and_deserialize()?;

                    let mut boards = HashMap::new();
                    for (url, id) in all_except_boards.board_urls.as_ref() {
                        let path = OpenBoardsArgs::board_path(save_path, *id);

                        let mut board_file = match std::fs::File::open(path) {
                            Ok(f) => f,
                            Err(_) => todo!("just remove board_id and add warn to log"),
                        };

                        board_file.read_to_end(&mut general_args.buf)?;
                        let board: Board = general_args.deserialize()?;
                        if &board.url != url { todo!("dyn id ? or hard err") }

                        boards.insert(*id, board);
                    }
                    let ignored = IgnoreOpenBoards { boards };

                    return Ok(all_except_boards.merge(ignored))
                }
            }
        }
    }
}