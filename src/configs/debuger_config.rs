pub trait MoniDebugerConfig {
    fn start_message(&self) -> String;
    fn success_message(&self) -> String;
    fn error_message(&self) -> String;
    fn execute_message(&self, command: &str) -> String;
    fn line_message(&self) -> String;
}
