use std::collections::{HashMap, HashSet};
pub use serde::Serialize;

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
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct BoardTag {
    pub tag: String,
}

pub struct OpenBoards {
    tagged_boards: HashMap<BoardTag, Vec<Board>>,
    other_boards: Vec<Board>,
    used_urls: HashSet<String>,
    board_qty: u32,
    pop_board_qty: u32,
}

impl OpenBoards {
    pub fn new() -> Self {
        Self {
            tagged_boards: HashMap::new(),
            other_boards: Vec::new(),
            used_urls: HashSet::new(),
            board_qty: 0,
            pop_board_qty: 0,
        }
    }

    /// # WARNING
    /// use only for init
    pub(in super) fn x_post_qty(&self) -> u64 {
        let mut ret = 0;
        for (_, bs) in &self.tagged_boards {
            bs.iter().for_each(|b|ret += b.post_qty);
        }
        self.other_boards.iter().for_each(|b|ret += b.post_qty);
        ret
    }

    pub fn open_boards_qty(&self) -> u32 {
        self.board_qty
    }
    pub fn pop_boards_qty(&self) -> u32 {
        self.pop_board_qty
    }

    /// # WARNING
    /// memorize result value! (until there no changes in `pop_boards_qty`)
    pub fn pop_boards(&self) -> Vec<PopBoardsTagInfo> {
        let mut ret = Vec::with_capacity(self.tagged_boards.len());
        for (tag, boards) in &self.tagged_boards {
            let boards = boards.iter().map(|b|PopBoardInfo{ 
                url: b.url.clone(), 
                name: b.name.clone(),
            }).collect();

            ret.push(PopBoardsTagInfo {
                tag: tag.tag.clone(),
                boards,
            })
        }
        ret
    }

    #[must_use]
    /// # Return value
    /// * `true` => sucessfully added
    /// * `false` => url already used for other board 
    pub fn add_board(&mut self, board: Board, tag: Option<BoardTag>) -> bool
    {
        if self.used_urls.contains(&board.url) { return false }
        self.used_urls.insert(board.url.clone());
        
        if let Some(tag) = tag {
            if let Some(boards) = self.tagged_boards.get_mut(&tag) {
                boards.push(board);
            } else {
                self.tagged_boards.insert(tag, vec![board]);
            }
            self.pop_board_qty += 1;
        } else {
            self.other_boards.push(board);
        }

        self.board_qty += 1;
        return true
    }

    pub fn is_board_exist(&self, board_url: &str) -> bool {
        self.used_urls.contains(board_url)
    }
}
