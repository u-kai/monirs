use clap::Parser;

use crate::{
    configs::moni_config::MoniConfig,
    parts::{debuger::MoniDebuger, moni_execute_command::MoniExecuteCommand},
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
    /// Sets the some ignore extensions split by comma or space
    #[clap(short = 'n', long)]
    ignore_extensions: Option<String>,
    /// Sets the some ignore path words split by comma or space
    #[clap(short = 'p', long)]
    ignore_path_words: Option<String>,
    /// Sets the execute command
    #[clap(short, long = "cmd")]
    execute_command: String,
}

fn split_space_or_comma<'a>(source: &'a str) -> Vec<&'a str> {
    if source.contains(",") {
        return source.split(",").collect();
    }
    if source.contains(" ") {
        return source.split(" ").map(|s| s.trim_end().trim()).collect();
    }
    vec![source]
}
impl<'a> MoniConfig<'a> for MoniCli {
    fn debug_message(&'a self) -> MoniDebuger {
        MoniDebuger::default()
    }
    fn execute_command(&'a self) -> MoniExecuteCommand<'a> {
        MoniExecuteCommand::new(&self.execute_command)
    }
    fn ignore_extensions(&'a self) -> Option<Vec<&'a str>> {
        if let Some(source) = &self.ignore_extensions {
            Some(split_space_or_comma(source))
        } else {
            None
        }
    }
    fn ignore_filenames(&'a self) -> Option<Vec<&'a str>> {
        if let Some(source) = &self.ignore_filenames {
            Some(split_space_or_comma(source))
        } else {
            None
        }
    }
    fn ignore_path_words(&'a self) -> Option<Vec<&'a str>> {
        if let Some(source) = &self.ignore_path_words {
            Some(split_space_or_comma(source))
        } else {
            None
        }
    }
    fn target_extensions(&'a self) -> Option<Vec<&'a str>> {
        if let Some(source) = &self.target_extensions {
            Some(split_space_or_comma(source))
        } else {
            None
        }
    }
    fn workspace(&'a self) -> Option<&'a str> {
        self.workspace.as_ref().map(|s| s.as_str())
    }
}
impl MoniCli {
    pub fn monitaring(&self) {
        let moni = self.to_moni();
        moni.monitaring()
    }
}

#[cfg(test)]
mod test_moni_cli_config {
    use super::*;
    impl MoniCli {
        fn new(
            workspace: &str,
            ignore_extensions: &str,
            ignore_filenames: &str,
            ignore_path_words: &str,
            target_extensions: &str,
            execute_command: &str,
        ) -> Self {
            Self {
                workspace: Some(workspace.to_string()),
                target_extensions: Some(target_extensions.to_string()),
                ignore_filenames: Some(ignore_filenames.to_string()),
                ignore_extensions: Some(ignore_extensions.to_string()),
                ignore_path_words: Some(ignore_path_words.to_string()),
                execute_command: execute_command.to_string(),
            }
        }
    }
    #[test]
    fn test_ignore_extensions_case_split_space() {
        let moni_cli = MoniCli::new("test", "py js", "", "", "", "");
        assert_eq!(moni_cli.ignore_extensions().unwrap(), vec!["py", "js"]);
        let moni_cli = MoniCli::new("test", "py js rs", "", "", "", "");
        assert_eq!(
            moni_cli.ignore_extensions().unwrap(),
            vec!["py", "js", "rs"]
        );
        let moni_cli = MoniCli::new("test", "py", "", "", "", "");
        assert_eq!(moni_cli.ignore_extensions().unwrap(), vec!["py"]);
    }
    #[test]
    fn test_ignore_extensions_case_split_comma() {
        let moni_cli = MoniCli::new("test", "py,js", "", "", "", "");
        assert_eq!(moni_cli.ignore_extensions().unwrap(), vec!["py", "js"]);
        let moni_cli = MoniCli::new("test", "py,js,rs", "", "", "", "");
        assert_eq!(
            moni_cli.ignore_extensions().unwrap(),
            vec!["py", "js", "rs"]
        );
        let moni_cli = MoniCli::new("test", "py", "", "", "", "");
        assert_eq!(moni_cli.ignore_extensions().unwrap(), vec!["py"]);
    }
}
