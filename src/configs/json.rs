use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::parts::{debuger::MoniDebuger, moni_execute_command::MoniExecuteCommand};

use super::{debuger_config::MoniDebugerConfig, moni_config::MoniConfig};
#[derive(Serialize, Deserialize)]
pub struct MoniJson {
    workspace: Option<String>,
    target_extensions: Option<Vec<String>>,
    ignore_filenames: Option<Vec<String>>,
    ignore_extensions: Option<Vec<String>>,
    ignore_path_words: Option<Vec<String>>,
    debug_message: Option<MoniDebugerConfigJson>,
    execute_command: String,
}
impl MoniJson {
    pub fn from_file<P: AsRef<Path> + Debug>(filepath: P) -> Result<Self, String> {
        if let Ok(file) = File::open(&filepath) {
            let mut json_content = String::new();
            let mut reader = BufReader::new(file);
            reader.read_to_string(&mut json_content).unwrap();
            let json_content: MoniJson = serde_json::from_str(&json_content).unwrap();
            Ok(json_content)
        } else {
            Err(format!("{:?} is not found", filepath))
        }
    }
    pub fn is_set_debug_message(&self) -> bool {
        self.debug_message.is_some()
    }
}
impl<'a> MoniConfig<'a> for MoniJson {
    fn execute_command(&'a self) -> MoniExecuteCommand<'a> {
        MoniExecuteCommand::new(&self.execute_command)
    }
    fn ignore_extensions(&'a self) -> Option<Vec<&'a str>> {
        opt_string_vec_to_str_vec(self.ignore_extensions.as_ref())
    }
    fn ignore_filenames(&'a self) -> Option<Vec<&'a str>> {
        opt_string_vec_to_str_vec(self.ignore_filenames.as_ref())
    }
    fn ignore_path_words(&'a self) -> Option<Vec<&'a str>> {
        opt_string_vec_to_str_vec(self.ignore_path_words.as_ref())
    }
    fn target_extensions(&'a self) -> Option<Vec<&'a str>> {
        opt_string_vec_to_str_vec(self.target_extensions.as_ref())
    }
    fn workspace(&'a self) -> Option<&'a str> {
        self.workspace.as_ref().map(|s| s.as_str())
    }
    fn debug_message(&'a self) -> MoniDebuger {
        if let Some(config) = self.debug_message.as_ref() {
            let title = if let Some(title) = config.title.as_ref() {
                title
            } else {
                "start monitaring"
            };
            let error = if let Some(error) = config.error.as_ref() {
                error
            } else {
                "error"
            };
            let exe_command = if let Some(exe_command) = config.execute.as_ref() {
                exe_command
            } else {
                "execute"
            };
            let success = if let Some(success) = config.success.as_ref() {
                success
            } else {
                "success"
            };
            let separator = "-";
            MoniDebuger::new(title, separator, success, error, exe_command)
        } else {
            MoniDebuger::default()
        }
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
pub struct MoniDebugerConfigJson {
    title: Option<String>,
    success: Option<String>,
    error: Option<String>,
    line: Option<String>,
    //$COMMAND
    execute: Option<String>,
}

impl MoniDebugerConfig for MoniDebugerConfigJson {
    fn error_message(&self) -> String {
        if let Some(error) = &self.error {
            error.to_owned()
        } else {
            " error ".to_string()
        }
    }
    fn execute_message(&self) -> String {
        if let Some(execute) = &self.execute {
            execute.to_owned()
        } else {
            " execute ".to_string()
        }
    }
    fn line_message(&self) -> String {
        if let Some(line) = &self.line {
            line.to_owned()
        } else {
            " --- ".to_string()
        }
    }
    fn start_message(&self) -> String {
        if let Some(start) = &self.title {
            start.to_owned()
        } else {
            " start monitaring ".to_string()
        }
    }
    fn success_message(&self) -> String {
        if let Some(success) = &self.success {
            success.to_owned()
        } else {
            " success ".to_string()
        }
    }
}
