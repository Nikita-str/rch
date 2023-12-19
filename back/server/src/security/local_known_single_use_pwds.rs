use std::collections::HashMap;
use std::io::{Write, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use strum::IntoEnumIterator;
use crate::utility::general::rand_string;

const PWD_LEN: usize = 12;
type Map = HashMap<Action, String>;

#[derive(strum_macros::EnumIter, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Action {
    FullSave = 1,
    FullLoad = 2,
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
    pub fn new<P>(output_path: &P) -> anyhow::Result<Self>
    where P: AsRef<std::path::Path>
    {
        let mut output = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(output_path)?;

        let act_to_pwd: Map = Action::iter()
            .map(|act|(act, rand_string(PWD_LEN)))
            .collect();
        
        for (act, pwd) in &act_to_pwd {
            output.write_all(&act.prefix())?;
            output.write_all(pwd.as_bytes())?;
            output.write_all(b"\n")?;
            output.flush()?;
        }

        Ok(Self {
            act_to_pwd,
            output,
        })
    }
    pub fn use_pwd_unchecked(&mut self, act: Action) -> anyhow::Result<()> {
            let new_pwd = rand_string(PWD_LEN);

            let index = act as usize - 1;
            let index = 1 /* Action byte */ + index * (1 /* Action byte */ + PWD_LEN + 1 /* `\n` */); 
            self.output.seek(SeekFrom::Start(index as u64))?;
            self.output.write_all(new_pwd.as_bytes())?;

            self.act_to_pwd.insert(act, new_pwd);
            Ok(())
    }
    pub fn use_pwd(&mut self, act: Action, pwd_query: &str) -> anyhow::Result<bool> {
        let correct = self.act_to_pwd.get(&act).map(|real_pwd|real_pwd == pwd_query);
        if correct == Some(true) {
            self.use_pwd_unchecked(act)?;
            return Ok(true)
        } else {
            return Ok(false)
        }
    }
    pub unsafe fn get_pwd(&self, act: Action) -> &str {
        self.act_to_pwd.get(&act).unwrap()
    }
}
