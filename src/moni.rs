use core::time;
use std::{
    fs::{File, Metadata},
    os::unix::prelude::MetadataExt,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    filesearcher::{FileSearcher, FileSearcherBuilder},
    filestore::FileStore,
};

#[derive(Debug)]
pub struct Moni<'a> {
    filestore: Arc<Mutex<FileStore>>,
    searcher: FileSearcher<'a>,
}
impl<'a> Moni<'a> {
    pub fn monitaring(&self) {
        loop {
            thread::sleep(time::Duration::new(1, 0));
            self.searcher
                .get_all_filenames()
                .into_iter()
                .map(|filepath| {
                    let file_path_str = filepath.as_str();
                    let meta = File::open(file_path_str)
                        .expect(&format!("{} not found", file_path_str))
                        .metadata()
                        .unwrap();
                    (filepath, to_num_time(meta))
                })
                .for_each(|(filepath, time)| {
                    let mut store = self.filestore.lock().unwrap();
                    if store.is_modify(&filepath, time) {
                        println!("{} modify", filepath);
                        store.update(filepath, time);
                        return;
                    }
                    if store.is_new(&filepath) {
                        println!("{} new", filepath);
                        store.insert(filepath, time);
                        return;
                    }
                })
        }
    }
}
pub struct MoniBuilder<'a> {
    searcher_builder: FileSearcherBuilder<'a>,
}

impl<'a> MoniBuilder<'a> {
    pub fn new() -> Self {
        Self {
            searcher_builder: FileSearcherBuilder::new(),
        }
    }
    pub fn build(self) -> Moni<'a> {
        let searcher = self.searcher_builder.build();
        let mut filestore = FileStore::new();
        searcher
            .get_all_filenames()
            .into_iter()
            .map(|filepath| {
                let file_path_str = filepath.as_str();
                let meta = File::open(file_path_str)
                    .expect(&format!("{} is not found", file_path_str))
                    .metadata()
                    .unwrap();
                (filepath, meta)
            })
            .for_each(|(path, meta)| filestore.insert(path, to_num_time(meta)));
        let filestore = Arc::new(Mutex::new(filestore));
        Moni {
            filestore,
            searcher,
        }
    }
    pub fn root(self, root: &'a str) -> Self {
        let searcher_builder = self.searcher_builder.root(root);
        Self { searcher_builder }
    }
    pub fn ignore_filename(self, filename: &'a str) -> Self {
        let searcher_builder = self.searcher_builder.ignore_filename(filename);
        Self { searcher_builder }
    }
    pub fn ignore_extension(self, extension: &'a str) -> Self {
        let searcher_builder = self.searcher_builder.ignore_extension(extension);
        Self { searcher_builder }
    }
    pub fn ignore_re(self, re: &'a str) -> Self {
        let searcher_builder = self.searcher_builder.ignore_re(re);
        Self { searcher_builder }
    }
}

fn to_num_time(metadata: Metadata) -> u128 {
    metadata.size() as u128
}
