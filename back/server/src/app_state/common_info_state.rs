use super::{SpeedPost, OpenBoards};

pub struct CommonInfoState {
    total_post: u64,
    open_boards: OpenBoards,
    speed_post: SpeedPost,
}

impl CommonInfoState {
    /// # params
    /// * `deleted_board_post`: qty of posts that was on board that already deleted 
    pub fn new(deleted_board_post: u64, open_boards: OpenBoards, speed_post: SpeedPost) -> Self {
        Self {
            total_post: open_boards.x_post_qty() + deleted_board_post,
            open_boards,
            speed_post,
        }
    }

    pub fn total_post(&self) -> u64 {
        self.total_post
    }
    pub fn speed_post(&self) -> u32 {
        self.speed_post.speed_post()
    }
    pub fn open_board_qty(&self) -> u32 {
        self.open_boards.open_boards_qty()
    }

    pub fn inc_post(&mut self) {
        self.speed_post.inc_post();
        self.total_post += 1;
    }
    pub fn upd_speed_post(&mut self) {
        self.speed_post.upd_speed_post()
    }
    
    pub fn mut_open_boards(&mut self) -> &mut OpenBoards {
        &mut self.open_boards
    }
    pub fn open_boards(&self) -> &OpenBoards {
        &self.open_boards
    }

    // pub fn pic_path(&self) -> String {
    //     self.open_boards.pic_path()
    // }    
    pub fn use_n_pic(&mut self, n: u64, board_url: &str) -> (String, std::ops::Range<u64>) {
        (self.open_boards.pic_path_unchecked(board_url), self.open_boards.use_n_pic(n))
    }

    pub fn pic_path_parent(&self) -> &str {
        self.open_boards.pic_path_parent()
    }
}

pub mod save_load {
    use super::*;
    use crate::utility::MutCell;
    use crate::utility::general::create_dir;
    use crate::utility::save_load::*;
    use crate::app_state::open_boards::save_load::OpenBoardsArgs;

    pub enum StateArgs {
        SingleFile(FileBufArgs),
        BoardSplited{
            general_args: FileBufArgs,
            save_path: String,
        },
    }
    impl StateArgs {
        fn split(&mut self) -> (&mut FileBufArgs, Option<&mut String>) {
            match self {
                StateArgs::SingleFile(save_args) => {
                    (save_args, None)
                }
                StateArgs::BoardSplited{ general_args, save_path }=> {
                    (general_args, Some(save_path))
                }
            }
        }

        fn merge_into_open_boards_args<'x>(
            args: &'x mut FileBufArgs, 
            path: Option<&'x mut String>,
        ) -> OpenBoardsArgs<'x> {
            match path {
                Some(save_path) => OpenBoardsArgs::BoardSplited {
                    general_args: MutCell::new_mut_ref(args),
                    save_path: MutCell::new_mut_ref(save_path),
                },
                None => OpenBoardsArgs::SingleFile(MutCell::new_mut_ref(args)),
            }
        }
    }

    impl Save<StateArgs> for CommonInfoState {
        fn save(&self, save_args: &mut StateArgs) -> anyhow::Result<()> {
            let (save_args, splited_board) = save_args.split();

            save_args.serialize_and_write(self.total_post)?;
            self.speed_post.save(save_args)?;

            let mut open_boards_save_args = 
                StateArgs::merge_into_open_boards_args(save_args, splited_board);
            self.open_boards.save(&mut open_boards_save_args)?;

            Ok(())
        }
    }

    impl Load<StateArgs> for CommonInfoState {
        fn load(load_args: &mut StateArgs) -> anyhow::Result<Self> {
            let (load_args, splited_board) = load_args.split();

            let total_post = load_args.read_and_deserialize()?;
            let speed_post = SpeedPost::load(load_args)?;

            let mut open_boards_load_args = 
                StateArgs::merge_into_open_boards_args(load_args, splited_board);
            let open_boards = OpenBoards::load(&mut open_boards_load_args)?;

            Ok(Self{
                total_post,
                open_boards,
                speed_post,
            })
        }
    }


    pub struct StateInitArgs<'dir> {
        pub save_dir: &'dir str,
        pub save_name: String,
        pub single_file: bool,
    }
    impl<'dir> StateInitArgs<'dir> {
        pub fn make_state_args(&self, is_save: bool) -> anyhow::Result<StateArgs> {
            match self.single_file {
                true => {
                    let path = format!("{}/{}.save", self.save_dir, self.save_name);
                    Ok(StateArgs::SingleFile(FileBufArgs::new(&path, is_save)?))
                }
                false => {
                    let save_path = format!("{}/{}", self.save_dir, self.save_name);
                    anyhow::ensure!(create_dir(&save_path), "failed to create save folder");
                    let path = format!("{save_path}/common.save");
                    Ok(StateArgs::BoardSplited {
                        general_args: FileBufArgs::new(&path, is_save)?, 
                        save_path,
                    })
                }
            }
        }
    }
    impl<'dir> Save<StateInitArgs<'dir>> for CommonInfoState {
        fn save(&self, args: &mut StateInitArgs<'dir>) -> anyhow::Result<()> {
            let mut args = args.make_state_args(true)?;
            self.save(&mut args)
        }
    }
    impl<'dir> Load<StateInitArgs<'dir>> for CommonInfoState {
        fn load(args: &mut StateInitArgs<'dir>) -> anyhow::Result<Self> {
            let mut args = args.make_state_args(false)?;
            Self::load(&mut args)
        }
    }

}