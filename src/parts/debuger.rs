use crate::configs::debuger_config::MoniDebugerConfig;

pub struct MoniDebuger<'a> {
    title: &'a str,
    separator: &'a str,
    success: &'a str,
    error: &'a str,
    execute: &'a str,
}
impl Default for MoniDebuger<'static> {
    fn default() -> Self {
        Self {
            title: "start",
            separator: "-----------------------",
            success: "success",
            error: "error",
            execute: "execute",
        }
    }
}
impl<'a> MoniDebuger<'a> {
    pub fn print_error_line(&self) -> () {
        println!("{}", self.error)
    }
    pub fn print_ok_line(&self) -> () {
        println!("{}", self.success)
    }
    pub fn print_line(&self) -> () {
        println!("{}", self.separator)
    }
    pub fn print_start_line(&self) -> () {
        println!("{}", self.title)
    }
    pub fn print_execute_command_line(&self, execute_command: &str) -> () {
        println!("{} {execute_command}", self.execute)
    }
}
impl<'a, C: MoniDebugerConfig<'a>> From<&'a C> for MoniDebuger<'a> {
    fn from(config: &'a C) -> Self {
        Self {
            title: config.start_message(),
            separator: config.line_message(),
            success: config.success_message(),
            error: config.error_message(),
            execute: config.execute_message(),
        }
    }
}
impl<'a> MoniDebuger<'a> {
    pub fn new(
        title: &'a str,
        separator: &'a str,
        success: &'a str,
        error: &'a str,
        execute: &'a str,
    ) -> Self {
        Self {
            title,
            separator,
            success,
            error,
            execute,
        }
    }
}

pub struct DefaultMoniDebugMessage<'a> {
    title: &'a str,
    separator: &'a str,
    separator_len: usize,
}
impl Default for DefaultMoniDebugMessage<'static> {
    fn default() -> Self {
        DefaultMoniDebugMessage {
            title: " start monitaring ",
            separator: "-",
            separator_len: 25,
        }
    }
}
impl<'a> DefaultMoniDebugMessage<'a> {
    fn calc_added_separator_len(&self, message: &str) -> usize {
        let diff = self.title.len() - message.len();
        let added_separator_len = self.separator_len + (diff / 2);
        added_separator_len
    }
    fn make_message(&self, message: &str) -> String {
        let separator_len = self.calc_added_separator_len(message);
        if separator_len % 2 == 0 {
            return format!(
                "{}{}{}-",
                self.separator.repeat(separator_len),
                message,
                self.separator.repeat(separator_len)
            );
        }
        format!(
            "{}{}{}",
            self.separator.repeat(separator_len),
            message,
            self.separator.repeat(separator_len)
        )
    }
    pub fn make_start_line_message(&self) -> String {
        let top_separator = self.separator.repeat(self.title.len());
        let bottom_separator = self.separator.repeat(self.title.len());
        format!(
            "\n{}\n{}\n{}\n",
            self.make_message(&top_separator),
            self.make_message(self.title),
            self.make_message(&bottom_separator)
        )
    }
    pub fn make_execute_command_line_message(&self) -> String {
        "execute ".to_string()
    }
    pub fn make_line_message(&self) -> String {
        let message = "--";
        self.make_message(message)
    }
    pub fn make_ok_line_message(&self) -> String {
        let message = " ok ";
        self.make_message(message)
    }
    pub fn make_error_line_message(&self) -> String {
        let message = " error ";
        self.make_message(message)
    }
    pub fn make_execute_line_message(&self) -> String {
        let message = " execute ";
        format!("\n{}\n", self.make_message(message),)
    }
}
