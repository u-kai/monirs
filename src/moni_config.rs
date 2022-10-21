use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::moni::{Moni, MoniBuilder, MoniPrinter};
pub trait MoniConfig<'a> {
    fn workspace(&'a self) -> Option<&'a str>;
    fn ignore_filenames(&'a self) -> Option<Vec<&'a str>>;
    fn ignore_extensions(&'a self) -> Option<Vec<&'a str>>;
    fn ignore_path_words(&'a self) -> Option<Vec<&'a str>>;
    fn target_extensions(&'a self) -> Option<Vec<&'a str>>;
    fn execute_command(&'a self) -> &'a str;
    fn to_moni<P: MoniPrinter>(&'a self, printer: P) -> Moni<'a, P> {
        let mut builder = MoniBuilder::new().exe_command(&self.execute_command());
        if self.ignore_filenames().is_some() {
            builder.set_ignore_files(self.ignore_filenames().unwrap())
        }
        if self.ignore_path_words().is_some() {
            builder.set_ignore_re(self.ignore_path_words().unwrap())
        }
        if self.target_extensions().is_some() {
            builder.set_target_extensions(self.target_extensions().unwrap())
        }
        if self.workspace().is_some() {
            builder.set_root(self.workspace().unwrap());
        } else {
            builder.set_root("./");
        }
        builder.build_with_printer(printer)
    }
}

pub struct MoniJsonConfig {
    json_content: MoniJson,
}

impl MoniJsonConfig {
    pub fn from_file<P: AsRef<Path> + Debug>(filepath: P) -> Result<Self, String> {
        if let Ok(file) = File::open(&filepath) {
            let mut json_content = String::new();
            let mut reader = BufReader::new(file);
            reader.read_to_string(&mut json_content).unwrap();
            let json_content: MoniJson = serde_json::from_str(&json_content).unwrap();
            Ok(Self { json_content })
        } else {
            Err(format!("{:?} is not found", filepath))
        }
    }
}

impl<'a> MoniConfig<'a> for MoniJsonConfig {
    fn execute_command(&'a self) -> &'a str {
        &self.json_content.execute_command
    }
    fn ignore_extensions(&'a self) -> Option<Vec<&'a str>> {
        opt_string_vec_to_str_vec(self.json_content.ignore_extensions.as_ref())
    }
    fn ignore_filenames(&'a self) -> Option<Vec<&'a str>> {
        opt_string_vec_to_str_vec(self.json_content.ignore_filenames.as_ref())
    }
    fn ignore_path_words(&'a self) -> Option<Vec<&'a str>> {
        opt_string_vec_to_str_vec(self.json_content.ignore_path_words.as_ref())
    }
    fn target_extensions(&'a self) -> Option<Vec<&'a str>> {
        opt_string_vec_to_str_vec(self.json_content.target_extensions.as_ref())
    }
    fn workspace(&'a self) -> Option<&'a str> {
        self.json_content.workspace.as_ref().map(|s| s.as_str())
    }
}

fn string_vec_to_str_vec<'a>(string_vec: &'a [String]) -> Vec<&'a str> {
    string_vec.iter().map(|s| s.as_str()).collect()
}
fn opt_string_vec_to_str_vec<'a>(string_vec: Option<&'a Vec<String>>) -> Option<Vec<&'a str>> {
    if let Some(string_vec) = string_vec {
        Some(string_vec_to_str_vec(string_vec))
    } else {
        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct MoniJson {
    workspace: Option<String>,
    target_extensions: Option<Vec<String>>,
    ignore_filenames: Option<Vec<String>>,
    ignore_extensions: Option<Vec<String>>,
    ignore_path_words: Option<Vec<String>>,
    execute_command: String,
}
