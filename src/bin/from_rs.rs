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
    let message = DefaultMoniDebugMessage::default();
    let debuger = MoniDebuger::from(message);
    monirs::moni::MoniBuilder::new()
        .root("./")
        .ignore_re("target")
        .exe_fn(exe_fn)
        .build_with_debuger(debuger)
        .monitaring();
}
