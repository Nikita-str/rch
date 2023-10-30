use std::collections::VecDeque;
use crate::post::Post;
use super::thread_usage_rate::ThreadUsageRate as ThrRate;

crate::define_id!(ThreadOpN: u64 [NO IMPL]);

/// after this post qty thr will be auto saged  
const BUMP_LIMIT: usize = 300;
// /// after this post qty thr will be deleted
// const DELETE_QTY: usize = WIPE_QTY * 2;

pub const MAX_HEADER_LEN: usize = 42;

pub struct Thread {
    op_n: ThreadOpN,
    header: String,
    /// * `VecDeque` for inf threads
    posts: VecDeque<Post>,
    infinity: bool,
}

impl Thread {
    fn ctor_valid_header<S: Into<String> + AsRef<str>>(header: S) -> String {
        let h = header.as_ref();
        if h.len() > MAX_HEADER_LEN {
            let mut header: String = h.chars().take(MAX_HEADER_LEN).collect();
            
            // correct split:
            let rest = &h[header.len()..];
            let correct_splited = rest.chars().next().map(|c|c.is_whitespace()).unwrap_or(true);
            if !correct_splited {
                if let Some(new_len) = header.rfind(char::is_whitespace) {
                    header.truncate(new_len)
                }
            }

            header
        } else {
            header.into()
        }
    }

    fn ctor_header_by_msg(post: &Post) -> String {
        Self::ctor_valid_header(post.text())
    }

    pub fn preproc_header(header: Option<String>, op_post: &Post) -> String {
        let header = header
            .map(|h|Self::ctor_valid_header(h))
            .unwrap_or_else(||Self::ctor_header_by_msg(op_post));
        header
    }

    // TODO: auth (before call this fn) when `infinity := true` 
    pub fn new(header: Option<String>, op_post: Post, infinity: bool) -> Self {
        let header = Self::preproc_header(header, &op_post);
        Self::new_preproced(header, op_post, infinity)
    }

    // TODO: auth (before call this fn) when `infinity := true` 
    pub fn new_preproced(header: String, op_post: Post, infinity: bool) -> Self {
        let op_n = op_post.n();
        assert!(op_n != 0);
        let op_n = ThreadOpN::from(op_n);

        let mut posts = VecDeque::with_capacity(BUMP_LIMIT);
        posts.push_back(op_post);

        Self {
            op_n,
            header,
            posts,
            infinity,
        }
    }

    pub fn add_post(&mut self, post: Post) {
        if self.infinity {
            if self.posts.len() == BUMP_LIMIT {
                // self.posts.pop_front();
                // save open post:
                self.posts.swap_remove_front(1);
            }
        }
        
        self.posts.push_back(post)
    }

    pub fn open_post(&self) -> &Post {
        self.posts.get(0).unwrap()
    }

    pub fn post(&self, n: usize) -> Option<&Post> {
        self.posts.get(n)
    }

    pub fn posts(&self, from: usize, n: usize) -> Vec<&Post> {
        let mut ret = Vec::with_capacity(n + 1);
        let to = from + n - 1;
        for post_n in from..=to {
            let Some(post) = self.post(post_n) else { break };
            ret.push(post)
        }
        ret
    }

    pub fn post_qty(&self) -> usize {
        self.posts.len()
    }
    
    pub fn header(&self) -> &str {
        &self.header
    }

    pub(in crate) fn op_n(&self) -> ThreadOpN {
        self.op_n
    }

    pub fn is_bump_limit_reached(&self) -> bool {
        if self.infinity { false } 
        else { self.posts.len() >= BUMP_LIMIT }
    }

    pub fn last_post_rate(&self) -> f32 {
        let post_n = self.posts.len() - 1;
        let mut iter = self.posts.iter();
        let post = iter.next_back().unwrap();
        let Some(prev_post) = iter.next_back() 
        else { return ThrRate::post_rate(post_n, 0.) };
        
        let dt_sec = post.dt(prev_post);
        ThrRate::post_rate(post_n, dt_sec)
    }
}