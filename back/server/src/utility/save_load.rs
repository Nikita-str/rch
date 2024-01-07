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
    pub fn serializer(&mut self) -> rmp_serde::Serializer<&mut Vec<u8>> {
        rmp_serde::Serializer::new(&mut self.buf)
    }
    pub fn serialize<S: serde::Serialize>(&mut self, obj: S) -> anyhow::Result<()> {
        let mut serializer = self.serializer();
        obj.serialize(&mut serializer)?;
        Ok(())
    }
    pub fn serialize_and_write<S: serde::Serialize>(&mut self, obj: S) -> anyhow::Result<()> {
        let mut serializer = self.serializer();
        obj.serialize(&mut serializer)?;
        self.write_len_and_buf()
    }
    pub fn deserialize<'x, D: serde::Deserialize<'x>>(&mut self) -> anyhow::Result<D> {
        let mut deserializer = rmp_serde::Deserializer::new(self.buf.as_slice());
        let obj = serde::Deserialize::deserialize(&mut deserializer)?;
        self.clear();
        Ok(obj)
    }
    pub fn read_and_deserialize<'x, D: serde::Deserialize<'x>>(&mut self) -> anyhow::Result<D> {
        self.read_len_and_buf()?;
        self.deserialize()
    }

    pub fn new(path: &str, is_save: bool) -> anyhow::Result<Self> {
        match is_save {
            true => Self::new_save(path),
            false => Self::new_load(path),
        }
    }
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
        
        self.clear();
        Ok(())
    }

    #[allow(clippy::uninit_vec)]
    pub fn read_exact_n_bytes(&mut self, n: usize) -> anyhow::Result<()> {
        self.buf.reserve(n);
        unsafe { self.buf.set_len(n); }
        self.file.read_exact(&mut self.buf[0..n])?;
        Ok(())
    }

    pub fn read_len_and_buf(&mut self) -> anyhow::Result<()> {
        let mut len = usize::to_le_bytes(0);
        self.file.read_exact(&mut len)?;
        let len = usize::from_le_bytes(len);

        self.read_exact_n_bytes(len)?;

        Ok(())
    }

    pub fn clear(&mut self) {
        self.buf.clear();
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
