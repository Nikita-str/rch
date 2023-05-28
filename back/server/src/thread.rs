use std::collections::VecDeque;
use crate::post::Post;
use super::thread_usage_rate::post_rate;

crate::define_id!(ThreadId);

/// after this post qty thr will be auto saged  
const BUMP_LIMIT: usize = 300;
// /// after this post qty thr will be deleted
// const DELETE_QTY: usize = WIPE_QTY * 2;

const MAX_HEADER_LEN: usize = 42;

pub struct Thread {
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

    // TODO: auth (before call this fn) when `infinity := true` 
    pub fn new(header: Option<String>, op_post: Post, infinity: bool) -> Self {
        let header = header
            .map(|h|Self::ctor_valid_header(h))
            .unwrap_or_else(||Self::ctor_header_by_msg(&op_post));

        let mut posts = VecDeque::with_capacity(BUMP_LIMIT);
        posts.push_back(op_post);

        Self {
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

    pub fn post(&self, n: usize) -> Option<&Post> {
        self.posts.get(n)
    }

    pub fn is_bump_limit_reached(&self) -> bool {
        if self.infinity { false } 
        else { self.posts.len() >= BUMP_LIMIT }
    }

    pub fn last_post_rate(&self) -> f32 {
        let post_n = self.posts.len() - 1;
        let mut iter = self.posts.iter();
        let post = iter.next_back().unwrap();
        let Some(prev_post) = iter.next_back() else { return post_rate(post_n, 0.) };
        
        let dt_sec = post.dt(prev_post);
        post_rate(post_n, dt_sec)
    }
}