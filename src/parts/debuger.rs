use crate::configs::debuger_config::MoniDebugerConfig;

pub struct MoniDebuger {
    title: String,
    separator: String,
    success: String,
    error: String,
    execute: String,
}
impl Default for MoniDebuger {
    fn default() -> Self {
        Self {
            title: "start".to_string(),
            separator: "-----------------------".to_string(),
            success: "success".to_string(),
            error: "error".to_string(),
            execute: "execute".to_string(),
        }
    }
}
impl MoniDebuger {
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
impl<'a, C: MoniDebugerConfig> From<&'a C> for MoniDebuger {
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
impl MoniDebuger {
    pub fn new<T: Into<String>>(title: T, separator: T, success: T, error: T, execute: T) -> Self {
        Self {
            title: title.into(),
            separator: separator.into(),
            success: success.into(),
            error: error.into(),
            execute: execute.into(),
        }
    }
}

pub struct DefaultMoniDebugMessage<'a> {
    title: &'a str,
    separator: &'a str,
    separator_len: usize,
}
impl<'a> MoniDebugerConfig for DefaultMoniDebugMessage<'a> {
    fn error_message(&self) -> String {
        self.make_error_line_message()
    }
    fn execute_message(&self) -> String {
        self.make_execute_command_line_message()
    }
    fn line_message(&self) -> String {
        self.make_line_message()
    }
    fn start_message(&self) -> String {
        self.make_start_line_message()
    }
    fn success_message(&self) -> String {
        self.make_ok_line_message()
    }
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
        let top_and_bottom = self.separator.repeat(separator_len);
        let diff = self.title.len() - message.len();
        if diff % 2 != 0 {
            return format!("{}{}{}-", top_and_bottom, message, top_and_bottom,);
        }
        format!("{}{}{}", top_and_bottom, message, top_and_bottom,)
    }
    pub fn make_start_line_message(&self) -> String {
        let top_and_bottom_separator = self.make_message(&self.separator.repeat(self.title.len()));
        format!(
            "\n{}\n{}\n{}\n",
            &top_and_bottom_separator,
            self.make_message(self.title),
            &top_and_bottom_separator,
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

#[cfg(test)]
mod test_debuger {
    use super::*;
    #[test]
    fn test_line_len() {
        let default_debuger = DefaultMoniDebugMessage::default();
        assert_eq!(
            default_debuger.error_message().len(),
            default_debuger.line_message().len()
        );
        assert_eq!(
            default_debuger.success_message().len(),
            default_debuger.line_message().len()
        );
    }
}
