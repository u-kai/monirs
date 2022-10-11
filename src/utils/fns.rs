use std::{
    fs::{self},
    path::{Path, PathBuf},
};

pub fn get_all_file_names(dir_path: impl AsRef<Path>) -> Result<Vec<PathBuf>, String> {
    match fs::read_dir(dir_path) {
        Ok(dir) => {
            let mut result = Vec::new();
            dir.filter_map(|entry| {
                if let Some(file_type_and_path) = entry.ok().map(|entry| match entry.file_type() {
                    Ok(_) => Some((entry.file_type().unwrap(), entry.path())),
                    _ => None,
                }) {
                    return file_type_and_path;
                };
                None
            })
            .for_each(|(file_type, path)| {
                if file_type.is_dir() {
                    let child_files = get_all_file_names(path).unwrap();
                    result.extend(child_files.into_iter());
                } else if file_type.is_file() {
                    result.push(path)
                }
            });
            Ok(result)
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
