use core::time;
#[cfg(not(target_os = "windows"))]
use std::os::unix::prelude::MetadataExt;
use std::{
    fs::{File, Metadata},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

use crate::configs::{debuger_config::MoniDebugerConfig, json::MoniDebugerConfigJson};

use super::{
    configs::{json::MoniJson, moni_config::MoniConfig},
    parts::{
        debuger::{DefaultMoniDebugMessage, MoniDebuger},
        filesearcher::{FileSearcher, FileSearcherBuilder},
        filestore::FileStore,
        moni_execute_command::MoniExecuteCommand,
    },
};

pub fn monitaring_from_json() -> () {
    let json = MoniJson::from_file("moni.json").unwrap();
    if json.is_set_debug_message() {
        json.to_moni().monitaring();
    } else {
        let message = DefaultMoniDebugMessage::default();
        let json_config = MoniDebugerConfigJson::from(message);
        let debuger = MoniDebuger::from(json_config);
        json.to_moni_with_debuger(debuger).monitaring()
    }
}
type CallBack = Box<dyn Fn(&str) -> Result<String, String>>;

pub struct Moni<'a, D: MoniDebugerConfig> {
    exe_command: Option<MoniExecuteCommand<'a>>,
    exe_fn: Option<CallBack>,
    filestore: Arc<Mutex<FileStore>>,
    searcher: FileSearcher<'a>,
    around_secs: u64,
    around_nanos: u32,
    debuger: MoniDebuger<D>,
}
impl<'a, C: MoniConfig<'a, D>, D: MoniDebugerConfig> From<&'a C> for Moni<'a, D> {
    fn from(config: &'a C) -> Self {
        let c = config.to_moni();
        c
    }
}

impl<'a, D: MoniDebugerConfig> Moni<'a, D> {
    pub fn monitaring(&self) {
        self.debuger.print_start_line();
        loop {
            thread::sleep(time::Duration::new(self.around_secs, self.around_nanos));
            self.searcher
                .get_all_filenames()
                .into_iter()
                .filter_map(|filepath| {
                    let file_path_str = filepath.as_str();
                    match get_file_size(file_path_str) {
                        Some(size) => Some((filepath, size)),
                        None => None,
                    }
                })
                .for_each(|(filepath, size)| {
                    let mut store = self.filestore.lock().unwrap();
                    if store.is_modify(&filepath, size) {
                        self.exe(&filepath);
                        store.update(filepath, size);
                        return;
                    }
                    if store.is_new(&filepath) {
                        self.exe(&filepath);
                        store.insert(filepath, size);
                        return;
                    }
                })
        }
    }
    fn exe(&self, filepath: &str) {
        if self.exe_fn.is_some() {
            match (self.exe_fn.as_ref().unwrap())(filepath) {
                Ok(result) => {
                    self.debuger.print_ok_line();
                    println!("{}", result);
                    self.debuger.print_line();
                }
                Err(e) => {
                    self.debuger.print_error_line();
                    println!("{}", e);
                    self.debuger.print_line();
                }
            };
            return;
        }
        if self.exe_command.is_some() {
            let exe_command = self
                .exe_command
                .as_ref()
                .unwrap()
                .to_execute_command(filepath);
            self.exe_command(&exe_command);
            return;
        }
    }

    fn exe_command(&self, exe_command: &str) {
        let mut command = target_os_command();
        command
            .arg("-c")
            .arg(exe_command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let status = command.status();
        let output = command.output();
        self.debuger.print_execute_command_line(exe_command);
        match status {
            Ok(status) if status.success() => {
                self.debuger.print_ok_line();
                println!("{}", String::from_utf8_lossy(&output.unwrap().stdout));
                self.debuger.print_line();
            }
            Ok(status) if !status.success() => {
                self.debuger.print_error_line();
                println!("{}", String::from_utf8_lossy(&output.unwrap().stderr));
                self.debuger.print_line();
            }
            Err(e) => {
                println!("{:#?}", e)
            }
            _ => {
                todo!("not impl case")
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn get_file_size(filepath: &str) -> Option<u128> {
    if let Ok(Ok(meta)) = File::open(filepath).map(|op| op.metadata()) {
        Some(meta_data_to_file_size(meta))
    } else {
        None
    }
}
#[cfg(target_os = "windows")]
fn get_file_size(filepath: &str) -> Option<u128> {
    use std::io::BufReader;
    use std::io::Read;
    if let Ok(file) = File::open(filepath) {
        let mut reader = BufReader::new(File::open(filepath).unwrap());
        let mut buf = Vec::new();
        let size = reader.read_to_end(&mut buf).unwrap() as u128;
        Some(size)
    } else {
        None
    }
}
pub struct MoniBuilder<'a> {
    exe_command: Option<MoniExecuteCommand<'a>>,
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
    #[cfg(target_os = "windows")]
    pub fn build_with_debuger<D: MoniDebugerConfig>(self, debuger: MoniDebuger<D>) -> Moni<'a, D> {
        use std::intrinsics::size_of;

        let searcher = self.searcher_builder.build();
        let mut filestore = FileStore::new();
        searcher
            .get_all_filenames()
            .into_iter()
            .filter_map(|filepath| {
                let file_path_str = filepath.as_str();
                (file_path_str, get_file_size(file_path_str))
            })
            .for_each(|(path, size)| filestore.insert(path, size));
        let filestore = Arc::new(Mutex::new(filestore));
        Moni {
            exe_command: self.exe_command,
            exe_fn: self.exe_fn,
            debuger,
            filestore,
            searcher,
            around_nanos: self.around_nanos,
            around_secs: self.around_secs,
        }
    }
    #[cfg(not(target_os = "windows"))]
    pub fn build_with_debuger<D: MoniDebugerConfig>(self, debuger: MoniDebuger<D>) -> Moni<'a, D> {
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
            .for_each(|(path, meta)| filestore.insert(path, meta_data_to_file_size(meta)));
        let filestore = Arc::new(Mutex::new(filestore));
        Moni {
            exe_command: self.exe_command,
            exe_fn: self.exe_fn,
            debuger,
            filestore,
            searcher,
            around_nanos: self.around_nanos,
            around_secs: self.around_secs,
        }
    }
    pub fn exe_fn<F>(mut self, exe_fn: F) -> Self
    where
        F: Fn(&str) -> Result<String, String> + 'static,
    {
        self.exe_fn = Some(Box::new(exe_fn));
        self
    }

    pub fn exe_command(mut self, exe_command: MoniExecuteCommand<'a>) -> Self {
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
    pub fn set_root(&mut self, root: &'a str) {
        self.searcher_builder.set_root(root);
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
    pub fn set_ignore_files(&mut self, filenames: Vec<&'a str>) {
        self.searcher_builder.set_ignore_files(filenames);
    }
    pub fn set_ignore_re(&mut self, re: Vec<&'a str>) {
        self.searcher_builder.set_ignore_re(re);
    }
    pub fn set_target_extensions(&mut self, target_extensions: Vec<&'a str>) {
        self.searcher_builder
            .set_target_extensions(target_extensions);
    }
}

#[cfg(not(target_os = "windows"))]
fn meta_data_to_file_size(metadata: Metadata) -> u128 {
    metadata.size() as u128
}

#[cfg(target_os = "linux")]
fn target_os_command() -> Command {
    Command::new("bash")
}
#[cfg(target_os = "windows")]
fn target_os_command() -> Command {
    Command::new("bash")
}
#[cfg(target_os = "macos")]
fn target_os_command() -> Command {
    Command::new("zsh")
}
