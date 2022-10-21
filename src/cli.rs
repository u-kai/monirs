use clap::Parser;

use crate::moni::{DefaultMoniPrinter, Moni, MoniBuilder};

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

impl MoniCli {
    pub fn monitaring(&self) {
        let d = self.workspace.as_ref();
        let printer = DefaultMoniPrinter::default();
        let mut builder = MoniBuilder::new().exe_command(&self.execute_command);
        //if self.ignore_filenames.is_some() {
        //let mut filenames = self.ignore_filenames.as_ref().unwrap();

        //builder.set_ignore_files(string_vec_to_str_vec(
        //self.ignore_filenames.as_ref().unwrap(),
        //))
        //}
        //if self.ignore_path_words.is_some() {
        //builder.set_ignore_re(string_vec_to_str_vec(
        //self.ignore_path_words.as_ref().unwrap(),
        //))
        //}
        //if self.target_extensions.is_some() {
        //builder.set_target_extensions(string_vec_to_str_vec(
        //self.target_extensions.as_ref().unwrap(),
        //))
        //}
        if self.workspace.is_some() {
            builder.set_root(self.workspace.as_ref().unwrap().as_str());
        } else {
            builder.set_root("./");
        }
        let moni = builder.build_with_printer(printer);
        moni.monitaring();
    }
}
fn string_vec_to_str_vec<'a>(string_vec: &'a [String]) -> Vec<&'a str> {
    string_vec.iter().map(|s| s.as_str()).collect()
}
