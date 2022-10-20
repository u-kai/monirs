use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::moni::{Moni, MoniBuilder, MoniPrinter};

pub trait MoniConfig {
    fn to_instance<'a, P: MoniPrinter>(&'a self, printer: P) -> Moni<'a, P>;
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

impl MoniConfig for MoniJsonConfig {
    fn to_instance<'a, P: MoniPrinter>(&'a self, printer: P) -> Moni<'a, P> {
        let mut builder = MoniBuilder::new().exe_command(&self.json_content.execute_command);
        if self.json_content.ignore_filenames.is_some() {
            builder.set_ignore_files(string_vec_to_str_vec(
                self.json_content.ignore_filenames.as_ref().unwrap(),
            ))
        }
        if self.json_content.ignore_path_words.is_some() {
            builder.set_ignore_re(string_vec_to_str_vec(
                self.json_content.ignore_path_words.as_ref().unwrap(),
            ))
        }
        if self.json_content.target_extensions.is_some() {
            builder.set_target_extensions(string_vec_to_str_vec(
                self.json_content.target_extensions.as_ref().unwrap(),
            ))
        }
        if self.json_content.workspace.is_some() {
            builder.set_root(self.json_content.workspace.as_ref().unwrap().as_str());
        } else {
            builder.set_root("./");
        }
        builder.build_with_printer(printer)
    }
}
fn string_vec_to_str_vec<'a>(string_vec: &'a [String]) -> Vec<&'a str> {
    string_vec.iter().map(|s| s.as_str()).collect()
}

#[derive(Serialize, Deserialize)]
pub struct MoniJson {
    workspace: Option<String>,
    target_extensions: Option<Vec<String>>,
    ignore_filenames: Option<Vec<String>>,
    ignore_path_words: Option<Vec<String>>,
    execute_command: String,
}
