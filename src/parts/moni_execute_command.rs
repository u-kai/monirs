pub struct MoniExecuteCommand<'a> {
    origin_command: &'a str,
}
impl<'a> MoniExecuteCommand<'a> {
    const MONI_MARK: &'static str = "MONI_FILE_PATH";
    pub fn new(origin_command: &'a str) -> Self {
        Self { origin_command }
    }
    pub fn to_execute_command(&self, filepath: &str) -> String {
        self.origin_command.replace(Self::MONI_MARK, filepath)
    }
}
#[cfg(test)]
mod test_moni_execute_command {
    use super::*;
    #[test]
    fn test_execute_command_case_replace() {
        let moni_exe = MoniExecuteCommand::new("python MONI_FILE_PATH");
        let filepath = "test.py";
        assert_eq!(
            moni_exe.to_execute_command(filepath),
            format!("python {}", filepath)
        )
    }
    #[test]
    fn test_execute_command_case_not_replace() {
        let moni_exe = MoniExecuteCommand::new("python test.py");
        let filepath = "test.py";
        assert_eq!(
            moni_exe.to_execute_command(filepath),
            format!("python test.py")
        )
    }
}
