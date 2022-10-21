use clap::Parser;

use crate::{
    moni::{DefaultMoniPrinter, Moni, MoniBuilder},
    moni_config::MoniConfig,
};

#[derive(Parser, Debug)]
#[clap(
    name = "moni",
    version = "1",
    author = "u-kai",
    about = "moni is monitaring your file change and execute command!"
)]
pub struct MoniCli {
    /// Sets the root monitaring directory
    #[clap(short, long)]
    workspace: Option<String>,
    /// Sets the some target extensions split by comma or space
    #[clap(short, long)]
    target_extensions: Option<String>,
    /// Sets the some ignore filenames split by comma or space
    #[clap(short = 'i', long)]
    ignore_filenames: Option<String>,
    /// Sets the some ignore path words split by comma or space
    #[clap(short = 'p', long)]
    ignore_path_words: Option<String>,
    /// Sets the execute command
    #[clap(short, long = "cmd")]
    execute_command: String,
}

//impl<'a> MoniConfig<'a> for MoniCli {
//fn execute_command(&'a self) -> &'a str {

//}
//}
impl MoniCli {
    pub fn monitaring(&self) {}
}
fn string_vec_to_str_vec<'a>(string_vec: &'a [String]) -> Vec<&'a str> {
    string_vec.iter().map(|s| s.as_str()).collect()
}
