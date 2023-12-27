use super::help::*;
use super::LoopActs;

pub struct ActionLooper {
    loop_acts: LoopActs,
    cmd_end_rx: Receiver<()>,
    cmd_done_sx: Sender<()>,
}

impl ActionLooper {
    pub fn new(loop_acts: LoopActs) -> ActionLooperCtrl {
        let (cmd_end_sx, cmd_end_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        let (cmd_done_sx, cmd_done_rx) = tokio::sync::mpsc::unbounded_channel::<()>();
        
        let x = Self {
            loop_acts,
            cmd_end_rx,
            cmd_done_sx,
        };

        let thr_loop_handler = x.loop_start();
        ActionLooperCtrl::new(cmd_end_sx, cmd_done_rx, thr_loop_handler)
    }

    fn loop_start(self) -> JoinHandle<()> {
        std::thread::spawn(move||{
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move { self.acts_loop().await })
        })
    }

    async fn acts_loop(mut self) {
        let mut index_awaiter = self.loop_acts.make_loop_acts_index();
        
        'del_loop: loop {
            tokio::select! {
                index = index_awaiter.await_next() => {    
                    match index {
                        Some(index) => self.loop_acts.act_step_upd(index, &mut index_awaiter),
                        None => println!("[ERR] index was `None`"),
                    }
                }
                Some(_) = self.cmd_end_rx.recv() => {
                    self.loop_acts.close_step();
                    break 'del_loop
                }
            }
        }
        let _ = self.cmd_done_sx.send(());
    }
}

pub struct ActionLooperCtrl {
    cmd_end_sx: Sender<()>,
    cmd_done_rx: Receiver<()>,

    thr_loop_handler: JoinHandle<()>, 
}

impl ActionLooperCtrl {
    fn new(
        cmd_end_sx: Sender<()>,
        cmd_done_rx: Receiver<()>,
        thr_loop_handler: JoinHandle<()>, 
    ) -> Self {
        Self {
            cmd_end_sx,
            cmd_done_rx,
            thr_loop_handler,
        }
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
