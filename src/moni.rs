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

type CallBack = Box<dyn Fn(&str) -> Result<(), String>>;

pub struct Moni<'a, P: MoniPrinter> {
    exe_command: Option<&'a str>,
    exe_fn: Option<CallBack>,
    filestore: Arc<Mutex<FileStore>>,
    searcher: FileSearcher<'a>,
    around_secs: u64,
    around_nanos: u32,
    printer: P,
}
impl<'a, P: MoniPrinter> Moni<'a, P> {
    pub fn monitaring(&self) {
        self.printer.print_start_line();
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
                        self.exe(&filepath);
                        store.update(filepath, time);
                        return;
                    }
                    if store.is_new(&filepath) {
                        self.exe(&filepath);
                        store.insert(filepath, time);
                        return;
                    }
                })
        }
    }
    fn exe(&self, filepath: &str) {
        if self.exe_fn.is_some() {
            match (self.exe_fn.as_ref().unwrap())(filepath) {
                Ok(_) => self.printer.print_ok_line(),
                Err(e) => {
                    self.printer.print_error_line();
                    println!("{}", e);
                }
            };
            return;
        }
        if self.exe_command.is_some() {
            self.exe_command(self.exe_command.as_ref().unwrap());
            return;
        }
    }
    fn exe_command(&self, exe_command: &str) {
        match Command::new("zsh")
            .arg("-c")
            .arg(exe_command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
        {
            Err(e) => println!("{:#?}", e),
            Ok(o) if o.status.success() => self.printer.print_ok_line(),
            Ok(o) if !o.status.success() => self.printer.print_error_line(),
            _ => self.printer.print_ok_line(),
        };
    }
}

pub trait MoniPrinter {
    fn print_start_line(&self) -> ();
    fn print_ok_line(&self) -> ();
    fn print_error_line(&self) -> ();
}
pub struct DefaultMoniPrinter<'a> {
    title: &'a str,
    separator: &'a str,
    separator_len: usize,
}
impl Default for DefaultMoniPrinter<'static> {
    fn default() -> Self {
        DefaultMoniPrinter {
            title: " start monitaring ",
            separator: "-",
            separator_len: 25,
        }
    }
}
impl<'a> DefaultMoniPrinter<'a> {
    fn calc_added_separator_len(&self, message: &str) -> usize {
        let diff = self.title.len() - message.len();
        let added_separator_len = self.separator_len + (diff / 2);
        added_separator_len
    }
    fn print_message(&self, message: &str) {
        let separator_len = self.calc_added_separator_len(message);
        if separator_len % 2 == 0 {
            println!(
                "{}-{}{}",
                self.separator.repeat(separator_len),
                message,
                self.separator.repeat(separator_len)
            );
            return;
        }
        println!(
            "{}{}{}",
            self.separator.repeat(separator_len),
            message,
            self.separator.repeat(separator_len)
        )
    }
}
impl<'a> MoniPrinter for DefaultMoniPrinter<'a> {
    fn print_start_line(&self) {
        let top_separator = self.separator.repeat(self.title.len());
        let bottom_separator = self.separator.repeat(self.title.len());
        println!();
        self.print_message(&top_separator);
        self.print_message(self.title);
        self.print_message(&bottom_separator);
        println!();
    }
    fn print_ok_line(&self) {
        let message = " ok ";
        self.print_message(message);
    }
    fn print_error_line(&self) {
        let message = " error ";
        self.print_message(message);
    }
}
pub struct MoniBuilder<'a> {
    exe_command: Option<&'a str>,
    exe_fn: Option<CallBack>,
    searcher_builder: FileSearcherBuilder<'a>,
    around_secs: u64,
    around_nanos: u32,
}

impl<'a> MoniBuilder<'a> {
    pub fn new() -> Self {
        Self {
            exe_command: None,
            exe_fn: None,
            around_nanos: 100_000_000,
            around_secs: 0,
            searcher_builder: FileSearcherBuilder::new(),
        }
    }
    pub fn build(self) -> Moni<'a, DefaultMoniPrinter<'a>> {
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
            exe_fn: self.exe_fn,
            printer: DefaultMoniPrinter::default(),
            filestore,
            searcher,
            around_nanos: self.around_nanos,
            around_secs: self.around_secs,
        }
    }
    pub fn exe_fn<F>(mut self, exe_fn: F) -> Self
    where
        F: Fn(&str) -> Result<(), String> + 'static,
    {
        self.exe_fn = Some(Box::new(exe_fn));
        self
    }
    pub fn exe_command(mut self, exe_command: &'a str) -> Self {
        self.exe_command = Some(exe_command);
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
