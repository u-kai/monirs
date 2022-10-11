use std::{ffi::OsStr, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Extension {
    Txt,
    Csv,
    Xlsx,
    Xlsm,
    Pptx,
    Bat,
    Java,
    Class,
    Json,
    Py,
    Rs,
    Ts,
    Js,
    Tsx,
    Jsx,
    Other,
}
impl Extension {
    pub fn new(path: &PathBuf) -> Result<Self, String> {
        if let Some(Some(extension)) = path.extension().map(|path| path.to_str()) {
            match extension {
                "txt" => Ok(Self::Txt),
                "csv" => Ok(Self::Csv),
                "xlsx" => Ok(Self::Xlsx),
                "xlsm" => Ok(Self::Xlsm),
                "pptx" => Ok(Self::Pptx),
                "bat" => Ok(Self::Bat),
                "java" => Ok(Self::Java),
                "class" => Ok(Self::Class),
                "json" => Ok(Self::Json),
                "py" => Ok(Self::Py),
                "rs" => Ok(Self::Rs),
                "ts" => Ok(Self::Ts),
                "js" => Ok(Self::Js),
                "tsx" => Ok(Self::Tsx),
                "jsx" => Ok(Self::Jsx),
                _ => Ok(Self::Other),
            }
        } else {
            Err(format!("{:#?} has not extension", path))
        }
    }
    pub fn is_match(&self, path: &PathBuf) -> bool {
        if let Some(Some(extension)) = path.extension().map(|path| path.to_str()) {
            self.as_str() == extension
        } else {
            false
        }
    }
    fn as_str(&self) -> &'static str {
        match *self {
            Self::Txt => "txt",
            Self::Csv => "csv",
            Self::Xlsx => "xlsx",
            Self::Xlsm => "xlsm",
            Self::Pptx => "pptx",
            Self::Bat => "bat",
            Self::Java => "java",
            Self::Class => "class",
            Self::Json => "json",
            Self::Py => "py",
            Self::Rs => "rs",
            Self::Ts => "ts",
            Self::Js => "js",
            Self::Tsx => "tsx",
            Self::Jsx => "jsx",
            Self::Other => "other",
        }
    }
}

#[cfg(test)]
mod extension_test {
    use super::*;
    #[test]
    fn test_case_rs() {
        let extension = Extension::new(&PathBuf::from("test/test.rs")).unwrap();
        assert!(extension.is_match(&PathBuf::from("test/test/test.rs")));
        assert!(!extension.is_match(&PathBuf::from("test/test/test.py")));
    }
}
