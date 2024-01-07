use std::path::PathBuf;

pub struct Executor {}

impl Executor {
    /// executor path, no file name
    pub fn path() -> PathBuf {
        let t = std::env::current_exe().expect("");
        t.parent().expect("").to_path_buf()
    }
    /// file name，no path，no extension name
    pub fn file_name() -> String {
        let mut t = std::env::current_exe().expect("");
        t.set_extension("");
        let t = t.file_name().expect("");
        t.to_owned().into_string().expect("")
    }
    /// file name，no path，no extension name
    pub fn file_name_with_extension(ext: &str) -> String {
        let mut t = std::env::current_exe().expect("");
        t.set_extension(ext);
        let t = t.file_name().expect("");
        t.to_owned().into_string().expect("")
    }
}