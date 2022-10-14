use std::{
    fs::{self},
    path::PathBuf,
    rc::Rc,
};

use regex::Regex;

use crate::extends::Extension;

pub struct FileSearcherBuilder<'a> {
    root: &'a str,
    ignore_re: Vec<Regex>,
    ignore_filenames: Vec<&'a str>,
    ignore_extension: Vec<Extension>,
}
impl<'a> FileSearcherBuilder<'a> {
    pub fn new() -> Self {
        Self {
            root: "./",
            ignore_re: Vec::new(),
            ignore_filenames: Vec::new(),
            ignore_extension: Vec::new(),
        }
    }
    pub fn build(self) -> FileSearcher<'a> {
        FileSearcher {
            root: self.root,
            ignore_re: Rc::new(self.ignore_re),
            ignore_filenames: Rc::new(self.ignore_filenames),
            ignore_extension: Rc::new(self.ignore_extension),
        }
    }
    pub fn root(mut self, root: &'a str) -> Self {
        self.root = root;
        self
    }
    pub fn ignore_filename(mut self, filename: &'a str) -> Self {
        self.ignore_filenames.push(filename);
        self
    }
    pub fn ignore_extension(mut self, extension: &str) -> Self {
        self.ignore_extension.push(Extension::from(extension));
        self
    }
    pub fn ignore_re(mut self, re: &'a str) -> Self {
        let regex = Regex::new(re).unwrap();
        self.ignore_re.push(regex);
        self
    }
}
pub struct FileSearcher<'a> {
    root: &'a str,
    ignore_re: Rc<Vec<Regex>>,
    ignore_filenames: Rc<Vec<&'a str>>,
    ignore_extension: Rc<Vec<Extension>>,
}
impl<'a> FileSearcher<'a> {
    pub fn spawn_child(&self, child_dir: &'a str) -> Self {
        FileSearcher {
            root: child_dir,
            ignore_re: self.ignore_re.clone(),
            ignore_filenames: self.ignore_filenames.clone(),
            ignore_extension: self.ignore_extension.clone(),
        }
    }
    pub fn get_all_filenames(&self) -> Vec<String> {
        let root_dir = fs::read_dir(self.root).expect(&format!("{} can not read_dir", self.root));
        let mut all_files = Vec::new();
        root_dir
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| match entry.file_type() {
                Ok(file_type) => Some((file_type, entry.path())),
                Err(_) => None,
            })
            .for_each(|(file_type, path)| {
                if !self.is_ignore(&path) {
                    let path = path.as_os_str().to_str().unwrap();
                    if file_type.is_dir() {
                        let child = self.spawn_child(path);
                        all_files.append(&mut child.get_all_filenames())
                    } else {
                        all_files.push(path.to_string());
                    }
                }
            });
        all_files
    }
    fn is_ignore(&self, path: &PathBuf) -> bool {
        let path_str = path.file_name().unwrap().to_str().unwrap();
        self.is_ignore_extension(path)
            || self.is_ignore_filename(path_str)
            || self.is_ignore_re(path_str)
    }
    fn is_ignore_extension(&self, path: &PathBuf) -> bool {
        self.ignore_extension
            .iter()
            .any(|extension| extension.is_match(path))
    }
    fn is_ignore_filename(&self, path: &str) -> bool {
        self.ignore_filenames.contains(&path)
    }
    fn is_ignore_re(&self, path: &str) -> bool {
        self.ignore_re.iter().any(|re| re.is_match(path))
    }
}

#[cfg(test)]
mod test_filesearcher {
    use super::*;
    #[test]
    fn test_get_all_filenames_by_use_preset_tests_dir_case_ignore_txt() {
        let filesearcher = FileSearcherBuilder::new()
            .root("./tests")
            .ignore_extension("txt")
            .build();
        let all_flies = filesearcher.get_all_filenames();
        let tobe_files = [
            "./tests/test.rs",
            "./tests/test1/test1-1/test1-1-1/test.txt",
            "./tests/test2/test2.txt",
            "./tests/test2/test2.md",
        ];
        for (i, file) in tobe_files.iter().enumerate() {
            if i == 0 {
                assert!(all_flies.contains(&file.to_string()))
            } else {
                assert!(!all_flies.contains(&file.to_string()))
            }
        }
    }
    #[test]
    fn test_get_all_filenames_by_use_preset_tests_dir_case_ignore_re() {
        let filesearcher = FileSearcherBuilder::new()
            .root("./tests")
            .ignore_re(r".rs")
            .build();
        let all_flies = filesearcher.get_all_filenames();
        let tobe_files = [
            "./tests/test.rs",
            "./tests/test1/test1-1/test1-1-1/test.txt",
            "./tests/test2/test2.txt",
        ];
        for (i, file) in tobe_files.iter().enumerate() {
            if i != 0 {
                assert!(all_flies.contains(&file.to_string()))
            } else {
                assert!(!all_flies.contains(&file.to_string()))
            }
        }
        let filesearcher = FileSearcherBuilder::new()
            .root("./tests")
            .ignore_re("test1")
            .build();
        let all_flies = filesearcher.get_all_filenames();
        let tobe_files = [
            "./tests/test.rs",
            "./tests/test1/test1-1/test1-1-1/test.txt",
            "./tests/test2/test2.txt",
        ];
        for (i, file) in tobe_files.iter().enumerate() {
            if i != 1 {
                assert!(all_flies.contains(&file.to_string()))
            } else {
                assert!(!all_flies.contains(&file.to_string()))
            }
        }
    }
    #[test]
    fn test_get_all_filenames_by_use_preset_tests_dir_case_ignore_filename() {
        let filesearcher = FileSearcherBuilder::new()
            .root("./tests")
            .ignore_filename("test2.txt")
            .build();
        let all_flies = filesearcher.get_all_filenames();
        let tobe_files = [
            "./tests/test.rs",
            "./tests/test1/test1-1/test1-1-1/test.txt",
            "./tests/test2/test2.txt",
        ];
        for (i, file) in tobe_files.iter().enumerate() {
            if i != 2 {
                assert!(all_flies.contains(&file.to_string()))
            } else {
                assert!(!all_flies.contains(&file.to_string()))
            }
        }
    }
    #[test]
    fn test_get_all_filenames_by_use_preset_tests_dir() {
        let filesearcher = FileSearcherBuilder::new().root("./tests").build();
        let all_flies = filesearcher.get_all_filenames();
        let tobe_files = [
            "./tests/test.rs",
            "./tests/test1/test1-1/test1-1-1/test.txt",
            "./tests/test2/test2.txt",
        ];
        println!("{:?}", all_flies);
        for file in tobe_files {
            assert!(all_flies.contains(&file.to_string()))
        }
    }
}
