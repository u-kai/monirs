use crate::{
    moni::{Moni, MoniBuilder},
    parts::{debuger::MoniDebuger, moni_execute_command::MoniExecuteCommand},
};

pub trait MoniConfig<'a> {
    fn workspace(&'a self) -> Option<&'a str>;
    fn ignore_filenames(&'a self) -> Option<Vec<&'a str>>;
    fn ignore_extensions(&'a self) -> Option<Vec<&'a str>>;
    fn ignore_path_words(&'a self) -> Option<Vec<&'a str>>;
    fn target_extensions(&'a self) -> Option<Vec<&'a str>>;
    fn execute_command(&'a self) -> MoniExecuteCommand<'a>;
    fn debug_message(&'a self) -> MoniDebuger;
    fn to_moni(&'a self) -> Moni<'a> {
        let debuger = self.debug_message();
        self.to_moni_with_debuger(debuger)
    }
    fn to_moni_with_debuger(&'a self, debuger: MoniDebuger) -> Moni<'a> {
        let mut builder = MoniBuilder::new().exe_command(self.execute_command());
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
        builder.build_with_debuger(debuger)
    }
}
