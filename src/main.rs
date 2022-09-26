use std::fs::{self, set_permissions, File};

use monirs::utils::fns::get_all_file_names;

fn main() {
    println!("{:?}", get_all_file_names("."))
}
