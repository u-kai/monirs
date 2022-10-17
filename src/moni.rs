use core::time;
use std::{
    fs::{File, Metadata},
    os::unix::prelude::MetadataExt,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    filesearcher::{FileSearcher, FileSearcherBuilder},
    filestore::FileStore,
};

#[derive(Debug)]
pub struct Moni<'a> {
    exe_command: &'a str,
    filestore: Arc<Mutex<FileStore>>,
    searcher: FileSearcher<'a>,
    around_secs: u64,
    around_nanos: u32,
}
impl<'a> Moni<'a> {
    pub fn monitaring(&self) {
        loop {
            thread::sleep(time::Duration::new(self.around_secs, self.around_nanos));
            self.searcher
                .get_all_filenames()
                .into_iter()
                .filter_map(|filepath| {
                    let file_path_str = filepath.as_str();
                    if let Ok(Ok(meta)) = File::open(file_path_str).map(|op| op.metadata()) {
                        Some((filepath, to_num_time(meta)))
                    } else {
                        None
                    }
                })
                .for_each(|(filepath, time)| {
                    let mut store = self.filestore.lock().unwrap();
                    if store.is_modify(&filepath, time) {
                        store.update(filepath, time);
                        self.exe_command();
                        return;
                    }
                    if store.is_new(&filepath) {
                        store.insert(filepath, time);
                        self.exe_command();
                        return;
                    }
                })
        }
    }
    fn exe_command(&self) {
        match Command::new("zsh")
            .arg("-c")
            .arg(self.exe_command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
        {
            Err(e) => println!("{:#?}", e),
            _ => println!("{}moni{}", "-".repeat(25), "-".repeat(25)),
        };
    }
}
pub struct MoniBuilder<'a> {
    exe_command: &'a str,
    searcher_builder: FileSearcherBuilder<'a>,
    around_secs: u64,
    around_nanos: u32,
}

impl<'a> MoniBuilder<'a> {
    pub fn new() -> Self {
        Self {
            around_nanos: 100_000_000,
            exe_command: "echo hello world",
            around_secs: 0,
            searcher_builder: FileSearcherBuilder::new(),
        }
    }
    pub fn build(self) -> Moni<'a> {
        let searcher = self.searcher_builder.build();
        let mut filestore = FileStore::new();
        searcher
            .get_all_filenames()
            .into_iter()
            .filter_map(|filepath| {
                let file_path_str = filepath.as_str();
                if let Ok(Ok(meta)) = File::open(file_path_str).map(|op| op.metadata()) {
                    Some((filepath, meta))
                } else {
                    None
                }
            })
            .for_each(|(path, meta)| filestore.insert(path, to_num_time(meta)));
        let filestore = Arc::new(Mutex::new(filestore));
        Moni {
            exe_command: self.exe_command,
            filestore,
            searcher,
            around_nanos: self.around_nanos,
            around_secs: self.around_secs,
        }
    }
    pub fn exe_command(mut self, exe_command: &'a str) -> Self {
        self.exe_command = exe_command;
        self
    }
    pub fn root(self, root: &'a str) -> Self {
        let searcher_builder = self.searcher_builder.root(root);
        Self {
            searcher_builder,
            ..self
        }
    }
    pub fn target_extension(self, extension: &'a str) -> Self {
        let searcher_builder = self.searcher_builder.target_extension(extension);
        Self {
            searcher_builder,
            ..self
        }
    }
    pub fn ignore_filename(self, filename: &'a str) -> Self {
        let searcher_builder = self.searcher_builder.ignore_filename(filename);
        Self {
            searcher_builder,
            ..self
        }
    }
    pub fn ignore_extension(self, extension: &'a str) -> Self {
        let searcher_builder = self.searcher_builder.ignore_extension(extension);
        Self {
            searcher_builder,
            ..self
        }
    }
    pub fn ignore_re(self, re: &'a str) -> Self {
        let searcher_builder = self.searcher_builder.ignore_re(re);
        Self {
            searcher_builder,
            ..self
        }
    }
}

fn to_num_time(metadata: Metadata) -> u128 {
    metadata.size() as u128
}
