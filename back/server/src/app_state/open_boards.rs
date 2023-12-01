use std::collections::{HashMap, HashSet};
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

pub struct Board {
    pub url: String,
    pub name: String,
    pub descr: String,
    pub post_qty: PostN,

    // TODO:MAYBE: SpeedPost
    thrs: HashMap<ThreadOpN, Thread>,
    thrs_usage: ThreadUsageRate, 
}

impl Board {
    pub fn next_post_n(&mut self) -> PostN {
        self.post_qty += 1;
        self.post_qty
    }

    pub fn new(url: String, name: String, descr: String, post_qty: PostN, max_thr_qty: usize) -> Self {
        Self {
            url,
            name,
            descr,
            post_qty,

            thrs: HashMap::new(),
            thrs_usage: ThreadUsageRate::new(max_thr_qty),
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
        assert!(self.thrs.insert(thr_op_n, Thread::new_preproced(header, op_post, infinity)).is_none());
        self.thrs_usage.add_new(thr_op_n);
        // TODO: delete least unrated if need
        thr_op_n.into()
    }

    #[allow(unused)]
    pub fn new_thr(&mut self, header: Option<String>, op_post: Post, infinity: bool) -> u64 {
        let header = Thread::preproc_header(header, &op_post);
        self.new_thr_preproced(header, op_post, infinity)
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

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct BoardTag {
    pub tag: String,
}

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

    pub fn pic_path(&self) -> String {
        self.pic_path.clone()
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
    pub fn add_board(&mut self, board: Board, tag: Option<BoardTag>) -> bool
    {
        const MAX_URL_LEN: usize = 16;
        if self.board_urls.contains_key(&board.url) { return false }
        if board.url.len() > MAX_URL_LEN { return false }

        self.board_qty += 1;
        if tag.is_some() { self.pop_board_qty += 1; }

        let id = self.next_board_id.inc();
        self.board_urls.insert(board.url.clone(), id);

        if let Some(boards) = self.board_tags.get_mut(&tag) {
            boards.push(id);
        } else {
            self.board_tags.insert(tag, vec![id]);
        }

        self.boards.insert(id, board);
        return true
    }

    pub fn is_board_exist(&self, board_url: &str) -> bool {
        self.board_urls.contains_key(board_url)
    }

    pub fn board_name(&self, board_url: &str) -> Option<&str> {
        self.board(board_url).map(|b|b.name.as_str())
    }

    pub fn board(&self, board_url: &str) -> Option<&Board> {
        self.board_urls.get(board_url).map(|id|self.boards.get(id).unwrap())
    }
    
    pub fn board_mut(&mut self, board_url: &str) -> Option<&mut Board> {
        self.board_urls.get(board_url).map(|id|self.boards.get_mut(id).unwrap())
    }
}
