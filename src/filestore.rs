use std::collections::HashMap;

type FileSize = u128;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileStore {
    store: HashMap<String, FileSize>,
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
    pub fn is_modify(&self, path: &str, filesize: FileSize) -> bool {
        if let Some(old_filesize) = self.store.get(path) {
            return old_filesize != &filesize;
        }
        false
    }
    pub fn update(&mut self, path: String, filesize: FileSize) {
        if self.is_modify(&path, filesize) {
            self.store.insert(path, filesize);
        }
    }
    pub fn insert(&mut self, path: String, filesize: FileSize) {
        if self.is_new(&path) {
            self.store.insert(path, filesize);
        }
    }
}
