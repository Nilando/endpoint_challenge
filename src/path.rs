use super::error::FileSystemError;
use std::fmt::Display;
use std::slice::IterMut;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Path {
    file_names: Vec<String>,
}

impl Path {
    pub fn new() -> Self {
        Self { file_names: vec![] }
    }

    pub fn push_file(&mut self, file: String) {
        self.file_names.push(file);
    }

    pub fn pop_file(&mut self) -> Option<String> {
        self.file_names.pop()
    }

    pub fn iter_mut(&mut self) -> IterMut<String> {
        self.file_names.iter_mut()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<Path> for String {
    fn from(value: Path) -> Self {
        let mut output = String::new();

        for file in value.file_names.iter() {
            output.push_str(file);
            output.push('/');
        }

        output.pop();

        output
    }
}

impl TryFrom<&str> for Path {
    type Error = FileSystemError;

    fn try_from(value: &str) -> Result<Self, FileSystemError> {
        if value == "/" {
            return Ok(Self { file_names: vec![] });
        }

        let file_names: Vec<String> = value.split('/').map(|s| s.to_string()).collect();

        for file_name in file_names.iter() {
            if file_name.len() == 0 {
                return Err(FileSystemError::BadPath(value.to_string()));
            }
        }

        Ok(Self { file_names })
    }
}
