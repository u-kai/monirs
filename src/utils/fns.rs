use std::{
    fs::{self, FileType},
    path::{Path, PathBuf},
};

pub fn get_all_file_names(dir_path: impl AsRef<Path>) -> Result<Vec<PathBuf>, String> {
    let dir = fs::read_dir(dir_path);
    match dir {
        Ok(dir) => {
            let file_names = dir
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| match entry.file_type() {
                    Ok(_) => Some((entry.file_type().unwrap(), entry.path())),
                    _ => None,
                })
                .flat_map(|(file_type, path)| {
                    if file_type.is_dir() {
                        get_all_file_names(path).unwrap()
                    } else if file_type.is_file() {
                        vec![path]
                    } else {
                        vec![]
                    }
                });
            Ok(file_names.collect())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod test_fns {
    use super::*;
    #[test]
    fn test_get_all_file_names_by_use_preset_tests_dir() {
        let file_names = get_all_file_names("./tests").unwrap();
        let tobe = vec![
            "./tests/test.rs",
            "./tests/test1/test1-1/test1-1-1/test.txt",
            "./tests/test2/test2.txt",
        ];
        let tobe = tobe.iter().map(|s| PathBuf::from(s)).collect::<Vec<_>>();
        assert_eq!(file_names, tobe);
    }
}
