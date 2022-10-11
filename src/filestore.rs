use std::{collections::HashMap, path::PathBuf};

type FileModifyTime = usize;
pub struct FileStore {
    store: HashMap<PathBuf, FileModifyTime>,
    ignore_paths: Vec<&'static str>,
    ignore_extends: Vec<Extends>,
}
impl FileStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            ignore_paths: Vec::new(),
            ignore_extends: Vec::new(),
        }
    }
    fn is_new(&mut self, path: &PathBuf) -> bool {
        self.store.contains_key(path)
    }
    fn is_modify(&mut self, path: &PathBuf, time: &FileModifyTime) -> bool {
        if let Some(old_time) = self.store.get(path) {
            return old_time <= time;
        }
        false
    }
    fn update(&mut self, path: PathBuf, time: FileModifyTime) {
        if self.is_modify(&path, &time) {
            self.store.insert(path, time);
        }
    }
    fn insert(&mut self, path: PathBuf, time: FileModifyTime) {
        if self.is_new(&path) {
            self.store.insert(path, time);
        }
    }
    fn add_ignore_path(&mut self, path: &'static str) {
        self.ignore_paths.push(path);
    }
    fn add_ignore_extends(&mut self, extends: Extends) {
        self.ignore_extends.push(extends);
    }
}

enum Extends {
    Txt,
    Csv,
    Xlsx,
    Xlsm,
    Pptx,
    Bat,
    Java,
    Class,
    Json,
    Py,
    Rs,
    Ts,
    Js,
    Tsx,
    Jsx,
}

#[cfg(test)]
mod moni_test {
    use super::*;
    #[test]
    fn test_case_use_igonre_extends() {
        let mut fs = FileStore::new();
        fs.add_ignore_extends(Extends::Txt);
        fs.add_ignore_extends(Extends::Csv);
        fs.insert(PathBuf::from("test/test.py"), 0);
        fs.insert(PathBuf::from("test/test.txt"), 0);
        fs.insert(PathBuf::from("test/test.csv"), 0);
        fs.insert(PathBuf::from("test/test/test/test.txt"), 0);
        fs.insert(PathBuf::from("test/test/test/test.csv"), 0);
        let mut tobe = FileStore::new();
        tobe.insert(PathBuf::from("test/test.py"), 0);
        tobe.insert(PathBuf::from("test/test.txt"), 0);
        tobe.insert(PathBuf::from("test/test.csv"), 0);
    }
}
