use std::{collections::HashMap, path::PathBuf};

type FileModifyTime = u128;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileStore {
    store: HashMap<String, FileModifyTime>,
}
impl FileStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    pub fn is_new(&self, path: &str) -> bool {
        !self.store.contains_key(path)
    }
    pub fn is_modify(&self, path: &str, time: FileModifyTime) -> bool {
        if let Some(old_time) = self.store.get(path) {
            println!("old is {} now is {}", old_time, time);
            return old_time != &time;
        }
        false
    }
    pub fn update(&mut self, path: String, time: FileModifyTime) {
        if self.is_modify(&path, time) {
            self.store.insert(path, time);
        }
    }
    pub fn insert(&mut self, path: String, time: FileModifyTime) {
        if self.is_new(&path) {
            self.store.insert(path, time);
        }
    }
}
