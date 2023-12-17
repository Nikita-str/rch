use std::fs::File;
use std::io::Write;
use std::io::Read;

pub struct NoArgs;

pub struct FileArgs {
    pub file: File,
}

pub struct FileBufArgs {
    /// file to save
    pub file: File,
    /// reusable between `save/load` call buffer <br/>
    /// it must be cleared on the end of call
    pub buf: Vec<u8>,
}
impl FileBufArgs {
    pub fn new_save(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            file: std::fs::File::create(path)?,
            buf: Vec::new(),
        })
    }
    pub fn new_load(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            file: std::fs::File::open(path)?,
            buf: Vec::new(),
        })
    }

    pub fn write_len_and_buf(&mut self) -> anyhow::Result<()> {
        let len = self.buf.len();
        let len_bytes = usize::to_le_bytes(len);
            
        self.file.write_all(&len_bytes)?;
        self.file.write_all(self.buf.as_slice())?;
        
        self.buf.clear();
        Ok(())
    }

    pub fn read_len_and_buf(&mut self) -> anyhow::Result<()> {
        let mut len = usize::to_le_bytes(0);
        self.file.read_exact(&mut len)?;
        let len = usize::from_le_bytes(len);

        self.buf.reserve(len);
        unsafe { self.buf.set_len(len); }
        self.file.read_exact(&mut self.buf[0..len])?;

        Ok(())
    }
}

pub trait Save<Args = FileBufArgs> {
    fn save(&self, save_args: &mut Args) -> anyhow::Result<()>;
}

pub trait Load<Args = FileBufArgs>
where Self: Sized {
    fn load(load_args: &mut Args) -> anyhow::Result<Self>;
}

// pub trait SaveLoad<SaveArgs = NoArgs, LoadArgs = SaveArgs> {
//     fn save(&self, save_args: SaveArgs);
//     fn load(load_args: SaveArgs) -> Self;
// }
