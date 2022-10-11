use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use crate::utils::fns::get_all_file_names;

type FileContent = String;
//pub struct Moni {
//files: Arc<Mutex<FileStore>>,
//}

//impl Moni {
//pub fn new() -> Self {
//Self {
//files: Arc::new(Mutex::new(FileStore::new())),
//}
//}
//pub fn monitaring<F>(&self, f: F)
//where
//F: Fn(&PathBuf, &str) -> (),
//{
//let file_paths = get_all_file_names("tests").unwrap();
//let mut handles = Vec::new();
//for path in file_paths {
//let file_map = self.files.clone();
//let handle = thread::spawn(move || {
//let mut content = String::new();
//let f = File::open(path.clone()).unwrap();
//println!("{:#?}", f.metadata());
//let mut reader = BufReader::new(File::open(path.clone()).unwrap());
//reader.read_to_string(&mut content).unwrap();
//let mut file_map = file_map.lock().unwrap();
//println!("path {:?} content {:?}", path, content);
////file_map.insert(path, content);
//});
//handles.push(handle);
//}
//for handle in handles {
//handle.join().unwrap();
//}
//}
//}
