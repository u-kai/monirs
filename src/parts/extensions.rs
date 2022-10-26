use std::path::PathBuf;

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
    Md,
    Other,
}
impl Extension {
    pub fn new(path: &PathBuf) -> Result<Self, String> {
        if let Some(Some(extension)) = path.extension().map(|path| path.to_str()) {
            Ok(Self::str_to_self(extension))
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
    fn str_to_self(extension: &str) -> Self {
        match extension {
            "txt" => Self::Txt,
            "csv" => Self::Csv,
            "xlsx" => Self::Xlsx,
            "xlsm" => Self::Xlsm,
            "pptx" => Self::Pptx,
            "bat" => Self::Bat,
            "java" => Self::Java,
            "class" => Self::Class,
            "json" => Self::Json,
            "py" => Self::Py,
            "rs" => Self::Rs,
            "ts" => Self::Ts,
            "js" => Self::Js,
            "tsx" => Self::Tsx,
            "jsx" => Self::Jsx,
            "md" => Self::Md,
            _ => Self::Other,
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
            Self::Md => "md",
            Self::Other => "other",
        }
    }
}

impl From<&str> for Extension {
    fn from(extension: &str) -> Self {
        Self::str_to_self(extension)
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
