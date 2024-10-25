use std::collections::BTreeMap;
use std::fmt::Display;
use super::path::Path;
use super::cmd::Cmd;
use super::error::FileSystemError;

struct Dir {
    path: Path,
    entries: BTreeMap<String, Dir>,
}

impl Dir {
    fn new(path: Path) -> Self {
        Self {
            path,
            entries: BTreeMap::new()
        }
    }

    fn create_dir(&mut self, name: String, dir: Dir) -> Result<(), FileSystemError> {
        if self.entries.get(&name).is_some() {
            let mut bad_path = self.path.clone();
            bad_path.push_file(name.clone());

            return Err(FileSystemError::FileAlreadyExists(bad_path));
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
                let move_file_name = src.pop_file().unwrap();

                // check if the dest exists before performing the move
                self.access_dir(dest.clone(), |dir| Ok(()))?; 

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
                let new_file = path.pop_file().unwrap();

                self.access_dir(path, |dir| {
                    dir.create_dir(new_file, new_dir)?;

                    Ok(None)
                })
            }
            Cmd::Delete(mut path) => {
                let file_name = path.pop_file().unwrap();

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

        for file_name in path.iter_mut() {
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

    #[test]
    fn move_non_existent_dir() {
        let mut fs = FileSystem::new();
        let path_a = Path::try_from("a").unwrap();
        let path_b = Path::try_from("b").unwrap();
        let path_c = Path::try_from("c").unwrap();
        
        fs.exec_cmd(Cmd::Create(path_a.clone())).unwrap();
        fs.exec_cmd(Cmd::Create(path_b.clone())).unwrap();

        assert!(fs.root.entries.len() == 2);
        assert!(fs.root.entries.get("a".into()).is_some());
        assert!(fs.root.entries.get("b".into()).is_some());

        let res = fs.exec_cmd(Cmd::Move { src: path_b, dest: path_c.clone() } );

        assert_eq!(res, Err(FileSystemError::PathDoesNotExist(path_c)));
        assert_eq!(fs.root.entries.len(), 2);
        assert!(fs.root.entries.get("a".into()).is_some());
        assert!(fs.root.entries.get("b".into()).is_some());
    }

    #[test]
    fn delete_non_existent_file() {
        let mut fs = FileSystem::new();
        let path_a = Path::try_from("a").unwrap();
        
        let res = fs.exec_cmd(Cmd::Delete(path_a.clone()));

        assert_eq!(res, Err(FileSystemError::PathDoesNotExist(path_a)));
    }

    #[test]
    fn move_to_root() {
        let mut fs = FileSystem::new();
        let path_a = Path::try_from("a").unwrap();
        let path_b = Path::try_from("a/b").unwrap();
        
        fs.exec_cmd(Cmd::Create(path_a.clone())).unwrap();
        fs.exec_cmd(Cmd::Create(path_b.clone())).unwrap();

        assert!(fs.root.entries.len() == 1);

        let dir_a = fs.root.entries.get("a".into()).unwrap();
        assert!(dir_a.entries.len() == 1);

        let dir_b = dir_a.entries.get("b".into()).unwrap();
        assert!(dir_b.entries.len() == 0);

        fs.exec_cmd(Cmd::Move { src: path_b, dest: Path::new() } ).unwrap();

        assert_eq!(fs.root.entries.len(), 2);
        assert!(fs.root.entries.get("a".into()).is_some());
        assert!(fs.root.entries.get("b".into()).is_some());
    }
}
