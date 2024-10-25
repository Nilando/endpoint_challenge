use std::collections::HashMap;

#[derive(Debug)]
pub enum FileSystemError {
    PathDoesNotExist,
    BadPath,
    CmdDoesNotExist,
    InvalidCmdArgs,
    InvalidProgramArgs,
    InternalError,
    FileAlreadyExists,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Path {
    file_names: Vec<String>
}

impl Path {
    const fn new() -> Self {
        Self {
            file_names: vec![]
        }
    }
}

impl TryFrom<&str> for Path {
    type Error = FileSystemError;

    fn try_from(value: &str) -> Result<Self, FileSystemError> {
        todo!()
    }
}

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
        todo!()
    }
}

struct Dir {
    entries: HashMap<String, Dir>,
}

impl Dir {
    fn new() -> Self {
        Self {
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
            return Err(FileSystemError::PathDoesNotExist);
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
            root: Dir::new(),
        }
    }

    pub fn exec_cmd(&mut self, cmd: Cmd) -> Result<Option<String>, FileSystemError> {
        match cmd {
            Cmd::List => {
                let mut output = String::new();

                self.traverse_dir(&self.root, 0, &mut |entry, depth| {
                    for _ in 0..depth {
                        output.push('\t');
                    }

                    output.push_str(entry);
                });

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
                let new_file = path.file_names.pop().unwrap();

                self.access_dir(path, |dir| {
                    dir.create_dir(new_file, Dir::new())?;

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
    fn access_dir<F, O>(&mut self, path: Path, cb: F) -> Result<O, FileSystemError> 
    where
        F: FnOnce(&mut Dir) -> Result<O, FileSystemError>,
    {
        todo!()
    }

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
