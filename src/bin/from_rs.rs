use std::{
    fs::File,
    io::{BufReader, Read},
};

use monirs::parts::debuger::{DefaultMoniDebugMessage, MoniDebuger};

fn main() {
    let exe_fn = |filepath: &str| -> Result<(), String> {
        let mut reader = BufReader::new(File::open(filepath).unwrap());
        let mut content = String::new();
        match reader.read_to_string(&mut content) {
            Err(e) => Err(e.to_string()),
            _ => {
                println!("file path is \n{}\n", filepath);
                println!("file content is \n{}\n", content);
                Ok(())
            }
        }
    };
    // tobe fix
    let message = DefaultMoniDebugMessage::default();
    let title = message.make_start_line_message();
    let separator = message.make_line_message();
    let success = message.make_ok_line_message();
    let error = message.make_error_line_message();
    let execute = message.make_execute_line_message();
    let debuger = MoniDebuger::new(&title, &separator, &success, &error, &execute);
    monirs::moni::MoniBuilder::new()
        .root("./")
        .ignore_re("target")
        .exe_fn(exe_fn)
        .build_with_debuger(debuger)
        .monitaring();
}
