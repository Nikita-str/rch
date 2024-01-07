use std::time::Duration;
use std::task::Poll;
use futures::{StreamExt, Future};
use futures::stream::FuturesUnordered;
use super::help::*;
use super::LoopDur;

struct LoopAct {
    act: Box<dyn LoopActor>,
    dur: Duration,
}

pub struct LoopActs {
    acts: Vec<LoopAct>,
}

impl LoopActs {
    pub fn new() -> Self {
        Self {
            acts: Vec::new(),
        }
    }
    pub fn add(&mut self, act: impl LoopActor + 'static, dur: Duration) {
        self.acts.push(LoopAct{
            act: Box::new(act),
            dur,
        })
    }
    pub fn make_loop_acts_index(&self) -> LoopActsIndAwaiter {
        LoopActsIndAwaiter::new(self)
    }

    pub fn act_step_upd(&mut self, index: usize, awaiter: &mut LoopActsIndAwaiter) {
        self.act_step(index);
        awaiter.add(IndexAwaiter::new(self.acts[index].dur, index));
    }
    pub fn act_step(&mut self, index: usize) {
        let act = &mut self.acts[index];
        act.act.act_step();
    }
    pub fn close_step(&mut self) {
        self.acts.iter_mut().for_each(|act|act.act.close_step());
    }
    pub fn init(&self) {
        self.acts.iter().for_each(|act|act.act.init());
    }
}

pub struct LoopActsIndAwaiter {
    index_futures: FuturesUnordered<IndexAwaiter>,
}
impl LoopActsIndAwaiter {
    fn new(acts: &LoopActs) -> Self {
        let index_futures: FuturesUnordered<_> = acts.acts.iter().enumerate()
            .map(|(index, act)|IndexAwaiter::new(act.dur, index))
            .collect();

        Self {
            index_futures
        }
    }
    pub async fn await_next(&mut self) -> Option<usize> {
        self.index_futures.next().await
    }
    fn add(&mut self, index_awaiter: IndexAwaiter) {
        self.index_futures.push(index_awaiter);
    }
}

pin_project_lite::pin_project! {
    pub struct IndexAwaiter {
        index: usize,
        #[pin]
        sleep: tokio::time::Sleep,
    }
}
impl IndexAwaiter {
    pub fn new(dur: Duration, index: usize) -> Self {
        Self {
            index,
            sleep: tokio::time::sleep(dur),
        }
    }
}
impl Future for IndexAwaiter {
    type Output = usize;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let proj = self.project();
        match proj.sleep.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => Poll::Ready(*proj.index),
        }
    }
}


pub struct ConfigCtorArgs<'x, S: Into<String>> {
    pub state: &'x crate::api::StdState,
    pub pic_path_parent: S,
}

impl<'x, S: Into<String>> crate::config::ConfigCtor<ConfigCtorArgs<'x, S>> for LoopActs {
    fn config_new(args: ConfigCtorArgs<S>) -> Self {
        let mut loop_acts = LoopActs::new();

        let act = super::SpeedPostUpdater::new(args.state);
        let dur = super::SpeedPostUpdater::config_loop_dur();
        loop_acts.add(act, dur);

        let pic_path_parent = args.pic_path_parent.into();
        let act = super::file_deleter::FileDelState::new(pic_path_parent);
        let dur = super::file_deleter::FileDelState::config_loop_dur();
        loop_acts.add(act, dur);

        let act = super::auto_saver::AutoSaver::new_std(args.state);
        let dur = super::auto_saver::AutoSaver::config_loop_dur();
        loop_acts.add(act, dur);

        loop_acts.init();
        loop_acts
    }
}