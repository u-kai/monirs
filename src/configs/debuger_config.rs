pub trait MoniDebugerConfig<'a> {
    fn start_message(&'a self) -> &'a str;
    fn success_message(&'a self) -> &'a str;
    fn error_message(&'a self) -> &'a str;
    fn execute_message(&'a self) -> &'a str;
    fn line_message(&'a self) -> &'a str;
}
