use std::collections::{VecDeque, HashSet};
use crate::post::{Post, PostN};
use super::thread_usage_rate::ThreadUsageRate as ThrRate;
use crate::utility::general as general;

crate::define_id!(ThreadOpN: u64 [NO IMPL]);

// /// after this post qty thr will be deleted
//TODO:MAYBE: const DELETE_QTY: usize = WIPE_QTY * 2;

#[inline]
fn bump_limit() -> usize {
    crate::config::config().imageboard.bump_limit
}
#[inline]
pub fn max_header_len() -> usize {
    crate::config::config().imageboard.max_header_len
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
struct ThreadPosts {
    //TODO: `bump_limit: usize`
    //      | because of already exisiting save we cant just add it here: 
    //      | we need version control for it 
    posts: VecDeque<Post>,
    posts_n: HashSet<PostN>,
}
impl ThreadPosts {
    pub fn __new_helper() -> Self {
        let posts = VecDeque::with_capacity(bump_limit());
        let posts_n = HashSet::with_capacity(bump_limit());        
        Self {
            posts,
            posts_n,
        }
    }

    pub fn new(op_post: Post) -> Self {
        let mut ret = Self::__new_helper();
        ret.push_back(op_post);
        ret
    }

    pub fn len(&self) -> usize {
        self.posts.len()
    }
    
    pub fn get(&self, index: usize) -> Option<&Post> {
        self.posts.get(index)
    }
    
    pub fn get_mut_by_n(&mut self, post_n: u64) -> Option<&mut Post> {
        match self.posts.binary_search_by(|post|post.n().cmp(&post_n)) {
            Ok(index) => Some(&mut self.posts[index]),
            Err(_) => None,
        }
    }
    
    pub fn push_back(&mut self, post: Post) {
        let post_n = post.n();
        self.posts.push_back(post);
        self.posts_n.insert(post_n);
    }

    pub fn swap_remove_front(&mut self, index: usize) -> Option<Post> {
        let post = self.posts.swap_remove_front(index);
        if let Some(post) = &post {
            self.posts_n.remove(&post.n());
        }
        post
    }
    
    pub fn iter_posts(&self) -> std::collections::vec_deque::Iter<Post> {
        self.posts.iter()
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
pub struct Thread {
    op_n: ThreadOpN,
    header: String,
    /// * `VecDeque` for inf threads
    posts: ThreadPosts,
    bump_time: general::Timestamp,
    infinity: bool,
}

impl Thread {
    fn ctor_valid_header<S: Into<String> + AsRef<str>>(header: S) -> String {
        let h = header.as_ref();
        if h.len() > max_header_len() {
            let mut header: String = h.chars().take(max_header_len()).collect();
            
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
        header
            .map(Self::ctor_valid_header)
            .unwrap_or_else(||Self::ctor_header_by_msg(op_post))
    }

    #[allow(unused)]
    // TODO: auth (before call this fn) when `infinity := true` 
    pub fn new(header: Option<String>, op_post: Post, infinity: bool) -> Self {
        let header = Self::preproc_header(header, &op_post);
        Self::new_preproced(header, op_post, infinity)
    }

    // TODO: auth (before call this fn) when `infinity := true` 
    pub fn new_preproced(header: String, op_post: Post, infinity: bool) -> Self {
        let op_n = op_post.n();
        assert!(op_n != 0);

        let bump_time = op_post.time();

        Self {
            op_n: ThreadOpN::from(op_n),
            posts: ThreadPosts::new(op_post),
            header,
            bump_time,
            infinity,
        }
    }

    pub fn add_post(&mut self, post: Post) {
        if self.infinity && (self.posts.len() >= bump_limit()) {
            // self.posts.pop_front();
            // save open post:
            self.posts.swap_remove_front(1);
        }
        
        if !self.is_bump_limit_reached() {
            self.bump_time = post.time();
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
        else { self.posts.len() >= bump_limit() }
    }

    pub fn bump_time(&self) -> general::Timestamp {
        self.bump_time
    }

    pub fn last_post_rate(&self) -> f32 {
        let post_n = self.posts.len() - 1;
        let mut iter = self.posts.iter_posts();
        let post = iter.next_back().unwrap();
        let Some(prev_post) = iter.next_back() 
        else { return ThrRate::post_rate(post_n, 0.) };
        
        let dt_sec = post.dt(prev_post);
        ThrRate::post_rate(post_n, dt_sec)
    }

    pub fn add_reply(&mut self, reply_from_post_n: PostN, reply_to_post_n: PostN) {
        if let Some(post) = self.posts.get_mut_by_n(reply_to_post_n) {
            post.add_reply(reply_from_post_n);
        }
    }
    pub fn add_replies<I, T>(&mut self, reply_from_post_n: PostN, reply_to_post_ns: I)
    where 
        I: IntoIterator<Item = T>,
        T: Into<u64>,
    {
        for reply_to_post_n in reply_to_post_ns {
            self.add_reply(reply_from_post_n, reply_to_post_n.into())    
        }
    }

    pub fn del<S: Into<String>>(self, board_url: S) {
        let mut pics_info = Vec::new();
        self.posts.posts.iter()
            .for_each(|post|post.add_del_info(&mut pics_info));
        if crate::utility::global_file_deleter::add_del_pics_act(board_url.into(), pics_info).is_err() {
            println!("[WARN] cant add del pic act")
        }
    }

    pub fn del_post_content<S>(&mut self, board_url: S, post_n: PostN) -> anyhow::Result<()>
    where S: Into<String> + AsRef<str>
    {
        let content = crate::preproc::pkg_del(board_url.as_ref());
        if self.op_n().0 == post_n {
            self.header = content.clone();
        }
        match self.posts.get_mut_by_n(post_n) {
            Some(post) => post.del_content(board_url, Some(content)),
            _ => anyhow::bail!("unknown post number {post_n}"),
        }
        Ok(())
    }
}
