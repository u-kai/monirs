use std::{collections::HashMap, path::PathBuf};

use crate::extends::Extension;

type FileModifyTime = usize;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileStore<'a> {
    store: HashMap<PathBuf, FileModifyTime>,
    ignore_paths: Vec<&'a str>,
    ignore_extends: Vec<Extension>,
}
impl<'a> FileStore<'a> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            ignore_paths: Vec::new(),
            ignore_extends: Vec::new(),
        }
    }
    fn is_new(&mut self, path: &PathBuf) -> bool {
        !self.store.contains_key(path)
    }
    fn is_modify(&mut self, path: &PathBuf, time: &FileModifyTime) -> bool {
        if let Some(old_time) = self.store.get(path) {
            return old_time <= time;
        }
        false
    }
    fn is_ignore(&self, path: &PathBuf) -> bool {
        self.ignore_extends
            .iter()
            .any(|extends| extends.is_match(path))
    }
    pub fn update(&mut self, path: PathBuf, time: FileModifyTime) {
        if self.is_modify(&path, &time) {
            self.store.insert(path, time);
        }
    }
    pub fn insert(&mut self, path: PathBuf, time: FileModifyTime) {
        if self.is_new(&path) && !self.is_ignore(&path) {
            self.store.insert(path, time);
        }
    }
    pub fn add_ignore_path(&mut self, path: &'a str) {
        self.ignore_paths.push(path);
    }
    pub fn add_ignore_extends(&mut self, extends: Extension) {
        self.ignore_extends.push(extends);
    }
}

#[cfg(test)]
mod moni_test {
    use crate::extends::Extension;

    use super::*;
    #[test]
    fn test_case_use_igonre_extends() {
        let mut fs = FileStore::new();
        fs.add_ignore_extends(Extension::Txt);
        fs.add_ignore_extends(Extension::Csv);
        fs.insert(PathBuf::from("test/test.py"), 0);
        fs.insert(PathBuf::from("test/test.txt"), 0);
        fs.insert(PathBuf::from("test/test.csv"), 0);
        fs.insert(PathBuf::from("test/test/test/test.txt"), 0);
        fs.insert(PathBuf::from("test/test/test/test.csv"), 0);
        let mut tobe = FileStore::new();
        tobe.insert(PathBuf::from("test/test.py"), 0);
        tobe.add_ignore_extends(Extension::Txt);
        tobe.add_ignore_extends(Extension::Csv);
        assert_eq!(fs, tobe);
    }
}
