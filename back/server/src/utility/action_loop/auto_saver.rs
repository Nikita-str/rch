use super::help::*;
use crate::utility::save_load::Save;

const H_TO_SEC: u64 = 60 * 60;
const SAVE_LOOP_SECS: u64 = 4 * H_TO_SEC; // 30;
pub const SAVE_LOOP_DUR: Duration = Duration::from_secs(SAVE_LOOP_SECS);
pub const STD_SAVE_NAMES: &[&str] = &["auto_save_A", "auto_save_B", "auto_save_C"];
pub const STD_CLOSE_SAVE_NAME: Option<&str> = Some("auto_save_X");

//TODO:MAYBE: GenericType for `save_obj`
pub struct AutoSaver {
    save_obj: crate::api::StdState,
    close_save_name: Option<String>,
    save_names: Vec<String>,
    last_save_index: usize,
    single_file: bool,
    close_single_file: bool,
}

impl AutoSaver {
    pub fn new(
        save_obj: &crate::api::StdState,
        close_save_name: Option<impl Into<String>>,
        save_names: &[&str],
        single_file: bool,
        close_single_file: bool,
    ) -> Self {
        Self {
            save_obj: Arc::clone(save_obj),
            close_save_name: close_save_name.map(|x|x.into()),
            save_names: save_names.into_iter().map(|x|x.to_string()).collect(),
            last_save_index: 0,
            single_file,
            close_single_file,
        }
    }
    pub fn new_std(save_obj: &crate::api::StdState) -> Self {
        let single_file = false;
        let close_single_file = true;
        Self::new(save_obj, STD_CLOSE_SAVE_NAME, STD_SAVE_NAMES, single_file, close_single_file)
    }

    fn get_save_name(&mut self) -> String {
        let save_name = self.save_names[self.last_save_index].clone();
        self.last_save_index += 1;
        if self.last_save_index >= self.save_names.len() {
            self.last_save_index = 0;
        }
        save_name
    }
    fn save(&mut self, save_name: String, single_file: bool) {
        println!("[INFO] [SAVE] save_name={save_name:?}; single_file={single_file:?};");
        
        let save_obj = match self.save_obj.write() {
            Ok(x) => x,
            Err(err) => {
                println!("[ERR] AutoSaver get `save_obj`: {err}");
                return
            }
        };
        let mut args = crate::api::ctrl::init_args(save_name, single_file);
        if let Err(err) = save_obj.save(&mut args) {
            println!("[ERR] AutoSaver `.save()`: {err}");
        }
    }
}

impl LoopActor for AutoSaver {
    fn act_step(&mut self) {
        let save_name = self.get_save_name();
        self.save(save_name, self.single_file);
    }
    
    fn close_step(&mut self) {
        if let Some(save_name) = &self.close_save_name {
            self.save(save_name.clone(), self.close_single_file)
        }
    }
    
    fn init(&self) {} 
}