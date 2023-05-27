use std::collections::{HashMap, VecDeque};
pub use serde::Serialize;
use crate::post::Post;
use crate::thread::{Thread, ThreadId};
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
    pub post_qty: u64,
    // TODO:MAYBE: SpeedPost
    thrs: HashMap<ThreadId, Thread>,
    thrs_usage: ThreadUsageRate, 
}

impl Board {
    pub fn new(url: String, name: String, descr: String, post_qty: u64, max_thr_qty: usize) -> Self {
        Self {
            url,
            name,
            descr,
            post_qty,
            thrs: HashMap::new(),
            thrs_usage: ThreadUsageRate::new(max_thr_qty),
        }
    }

    pub fn top_thr_by_usage_n(&self, n: usize) -> Option<&Thread> {
        self.thrs_usage.top_n(n).map(|id|self.thrs.get(&id).unwrap())
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
}

impl OpenBoards {
    pub fn new() -> Self {
        Self {
            boards: HashMap::new(),
            board_tags: HashMap::new(),
            board_urls: HashMap::new(),
            next_board_id: BoardId::first(),
            board_qty: 0,
            pop_board_qty: 0,
        }
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

    pub fn board(&self, board_url: &str) -> Option<&Board> {
        self.board_urls.get(board_url).map(|id|self.boards.get(id).unwrap())
    }
}
