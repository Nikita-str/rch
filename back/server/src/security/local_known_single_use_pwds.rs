use std::collections::HashMap;
use std::io::{Write, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use strum::IntoEnumIterator;
use crate::utility::general::rand_string;

const NONCE_LEN: usize = 8;
const PWD_LEN: usize = 12;
type Map = HashMap<Action, (String, String)>;

#[derive(strum_macros::EnumIter, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Action {
    FullSave = 1,
    FullLoad = 2,
    AddBoard = 3,
    DelPost = 4,
    DelThr = 5,
    Shutdown = 6,
}
impl Action {
    const fn prefix(self) -> [u8; 1] {
        [self as u8]
    }
}

pub struct SingleUsePwds {
    act_to_pwd: Map,
    output: File,
}
impl SingleUsePwds {
    fn nonce_pwd_pair() -> (String, String) {
        (rand_string(NONCE_LEN), rand_string(PWD_LEN))
    }

    pub fn new<P>(output_path: &P) -> anyhow::Result<Self>
    where P: AsRef<std::path::Path>
    {
        let mut output = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(output_path)?;

        let act_to_pwd: Map = Action::iter()
            .map(|act|(act, Self::nonce_pwd_pair()))
            .collect();
        
        for act in Action::iter() {
            let (nonce, pwd) = act_to_pwd.get(&act).unwrap();
            output.write_all(&act.prefix())?;
            output.write_all(b" ")?;
            output.write_all(nonce.as_bytes())?;
            output.write_all(b" ")?;
            output.write_all(pwd.as_bytes())?;
            output.write_all(b"\n")?;
            output.flush()?;
        }

        Ok(Self {
            act_to_pwd,
            output,
        })
    }
    pub unsafe fn use_pwd_unchecked(&mut self, act: Action) -> anyhow::Result<()> {
            let (new_nonce, new_pwd) = Self::nonce_pwd_pair();

            let index = act as usize - 1;
            let index = 1 /* Action byte */ + 1 /* space */ +  index * (
                1 /* Action byte */ + 
                1 /* space */ + 
                NONCE_LEN + 
                1 /* space */ + 
                PWD_LEN + 
                1 /* `\n` */
            ); 
            self.output.seek(SeekFrom::Start(index as u64))?;
            self.output.write_all(new_nonce.as_bytes())?;
            self.output.write_all(b" ")?;
            self.output.write_all(new_pwd.as_bytes())?;
            self.output.flush()?;

            self.act_to_pwd.insert(act, (new_nonce, new_pwd));
            Ok(())
    }
    pub fn use_pwd(
        &mut self, 
        act: Action, 
        act_nonce: &str,
        hash_expected: &[u8],
    )-> anyhow::Result<bool> {
        use sha3::Digest;
        let mut hasher = sha3::Sha3_256::new();
        let correct = self.act_to_pwd.get(&act).map(|(nonce, pwd)|{
            hasher.update(nonce);
            hasher.update(act_nonce);
            hasher.update(pwd);
            let real_hash = &hasher.finalize()[..];
            real_hash == hash_expected
        });
        if correct == Some(true) {
            unsafe { self.use_pwd_unchecked(act)?; }
            return Ok(true)
        } else {
            return Ok(false)
        }
    }
    // pub unsafe fn get_nonce_pwd(&self, act: Action) -> (&str, &str) {
    //     self.act_to_pwd.get(&act).map(|(a, b)|(a.as_str(), b.as_str())).unwrap()
    // }
}
