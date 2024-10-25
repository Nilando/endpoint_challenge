use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub enum FileSystemError {
    PathDoesNotExist(Path),
    BadPath,
    CmdDoesNotExist,
    InvalidCmdArgs,
    InvalidProgramArgs,
    InternalError,
    FileAlreadyExists,
}

impl Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PathDoesNotExist(path) => {
                write!(f, "{} does not exist", String::from(path.clone()))
            }
            _ => {
                todo!()
            }
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Path {
    file_names: Vec<String>
}

impl Path {
    fn new() -> Self {
        Self {
            file_names: vec![]
        }
    }

    fn push_file(&mut self, file: String) {
        self.file_names.push(file);
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
        let file_names: Vec<String> = value
            .split('/')
            .map(|s| s.to_string())
            .collect();

        for file_name in file_names.iter() {
            if file_name.len() == 0 {
                return Err(FileSystemError::BadPath);
            }
        }

        Ok(Self { file_names })
    }
}

#[derive(Clone)]
pub enum Cmd {
    Move {
        src: Path, 
        dest: Path
    },
    Create(Path),
    Delete(Path),
    List,
}

impl TryFrom<&str> for Cmd {
    type Error = FileSystemError;

    fn try_from(value: &str) -> Result<Self, FileSystemError> {
        let args: Vec<String> = value
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        match args[0].as_str() {
            "MOVE" => {
                if args.len() != 3 {
                    return Err(FileSystemError::InvalidCmdArgs);
                }

                Ok(
                    Cmd::Move {
                        src: Path::try_from(args[1].as_str())?,
                        dest: Path::try_from(args[2].as_str())?,
                    }
                )
            }
            "CREATE" => {
                if args.len() != 2 {
                    return Err(FileSystemError::InvalidCmdArgs);
                }

                let path = Path::try_from(args[1].as_str())?;

                Ok(Cmd::Create(path))
            }
            "DELETE" => {
                if args.len() != 2 {
                    return Err(FileSystemError::InvalidCmdArgs);
                }

                let path = Path::try_from(args[1].as_str())?;

                Ok(Cmd::Delete(path))
            }
            "LIST" => {
                if args.len() != 1 {
                    return Err(FileSystemError::InvalidCmdArgs);
                }

                Ok(Cmd::List)
            }
            _ => Err(FileSystemError::CmdDoesNotExist),
        }
    }
}

struct Dir {
    path: Path,
    entries: HashMap<String, Dir>,
}

impl Dir {
    fn new(path: Path) -> Self {
        Self {
            path,
            entries: HashMap::new()
        }
    }

    fn create_dir(&mut self, name: String, dir: Dir) -> Result<(), FileSystemError> {
        if self.entries.get(&name).is_some() {
            return Err(FileSystemError::FileAlreadyExists);
        }

        self.entries.insert(name, dir);

        Ok(())
    }

    fn delete(&mut self, name: &String) -> Result<Dir, FileSystemError> {
        if self.entries.get(name).is_none() {
            let mut bad_path = self.path.clone();
            bad_path.push_file(name.clone());

            return Err(FileSystemError::PathDoesNotExist(bad_path));
        }

        let dir = self.entries.remove(name).unwrap();

        Ok(dir)
    }
}

pub struct FileSystem {
    root: Dir,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: Dir::new(Path::new()),
        }
    }

    /// Attempts to execute a command.
    pub fn exec_cmd(&mut self, cmd: Cmd) -> Result<Option<String>, FileSystemError> {
        match cmd {
            Cmd::List => {
                let mut output = String::new();

                self.traverse_dir(&self.root, 0, &mut |entry, depth| {
                    for _ in 0..depth {
                        output.push_str("  ");
                    }

                    output.push_str(entry);

                    output.push('\n');
                });

                output.pop();

                Ok(Some(output))
            }
            Cmd::Move { mut src, dest } => {
                let move_file_name = src.file_names.pop().unwrap();

                let move_dir: Dir = 
                    self.access_dir(src, |dir| {
                        Ok(dir.delete(&move_file_name)?)
                    })?;

                self.access_dir(dest, |dir| {
                    dir.create_dir(move_file_name, move_dir)?;

                    Ok(None)
                })
            }
            Cmd::Create(mut path) => {
                let new_dir = Dir::new(path.clone());
                let new_file = path.file_names.pop().unwrap();

                self.access_dir(path, |dir| {
                    dir.create_dir(new_file, new_dir)?;

                    Ok(None)
                })
            }
            Cmd::Delete(mut path) => {
                let file_name = path.file_names.pop().unwrap();

                self.access_dir(path, |dir| {
                    dir.delete(&file_name)?;

                    Ok(None)
                })
            }
        }
    }

    /// Attempts to follow a path in the filesystem.
    ///
    /// May return PathDoesNotExist if path cannot be followed.
    ///
    /// Returns true if the final directory in the path exists, false it not.
    fn access_dir<F, O>(&mut self, mut path: Path, cb: F) -> Result<O, FileSystemError> 
    where
        F: FnOnce(&mut Dir) -> Result<O, FileSystemError>,
    {
        let mut current_dir = &mut self.root;
        let mut current_path = Path::new();

        for file_name in path.file_names.iter_mut() {
            current_path.push_file(file_name.to_string());
            match current_dir.entries.get_mut(file_name) {
                Some(dir) => {
                    current_dir = dir;
                }
                None => {
                    return Err(FileSystemError::PathDoesNotExist(path))
                }
            }
        }

        cb(&mut current_dir)
    }

    /// Breath first traversal of a directory.
    ///
    /// The provided callback is called for every directory found
    /// and passes in the "depth" of the folder and its file name as args.
    fn traverse_dir(&self, dir: &Dir, depth: usize, cb: &mut impl FnMut(&str, usize)) {
        for (entry, dir) in dir.entries.iter() {
            cb(entry, depth);

            self.traverse_dir(dir, depth + 1, cb);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_empty_dir() {
        let mut fs = FileSystem::new();
        let result = fs.exec_cmd(Cmd::List).unwrap();
        let expect = Some("".into());

        assert_eq!(result, expect);
    }

    #[test]
    fn create_and_delete_directory() {
        let mut fs = FileSystem::new();
        let path = Path::try_from("test").unwrap();
        
        fs.exec_cmd(Cmd::Create(path.clone())).unwrap();

        assert!(fs.root.entries.len() == 1);
        assert!(fs.root.entries.get("test".into()).is_some());

        fs.exec_cmd(Cmd::Delete(path)).unwrap();

        assert!(fs.root.entries.len() == 0);
        assert!(fs.root.entries.get("test".into()).is_none());
    }

    #[test]
    fn move_directory() {
        let mut fs = FileSystem::new();
        let path_a = Path::try_from("a").unwrap();
        let path_b = Path::try_from("b").unwrap();
        
        fs.exec_cmd(Cmd::Create(path_a.clone())).unwrap();
        fs.exec_cmd(Cmd::Create(path_b.clone())).unwrap();

        assert!(fs.root.entries.len() == 2);
        assert!(fs.root.entries.get("a".into()).is_some());
        assert!(fs.root.entries.get("b".into()).is_some());

        fs.exec_cmd(Cmd::Move { src: path_b, dest: path_a } ).unwrap();

        assert!(fs.root.entries.len() == 1);

        let dir_a = fs.root.entries.get("a".into()).unwrap();

        assert!(dir_a.entries.len() == 1);

        let dir_b = dir_a.entries.get("b".into()).unwrap();

        assert!(dir_b.entries.len() == 0);
    }
}
