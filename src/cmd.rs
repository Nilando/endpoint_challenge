use super::error::FileSystemError;
use super::path::Path;

#[derive(Clone)]
pub enum Cmd {
    Move { src: Path, dest: Path },
    Create(Path),
    Delete(Path),
    List,
}

impl TryFrom<&str> for Cmd {
    type Error = FileSystemError;

    fn try_from(value: &str) -> Result<Self, FileSystemError> {
        let args: Vec<String> = value.split_whitespace().map(|s| s.to_string()).collect();

        match args[0].as_str() {
            "MOVE" => {
                if args.len() != 3 {
                    return Err(FileSystemError::InvalidCmdArgs(
                        "usage: MOVE src dest".to_string(),
                    ));
                }

                Ok(Cmd::Move {
                    src: Path::try_from(args[1].as_str())?,
                    dest: Path::try_from(args[2].as_str())?,
                })
            }
            "CREATE" => {
                if args.len() != 2 {
                    return Err(FileSystemError::InvalidCmdArgs(
                        "usage: CREATE file".to_string(),
                    ));
                }

                let path = Path::try_from(args[1].as_str())?;

                Ok(Cmd::Create(path))
            }
            "DELETE" => {
                if args.len() != 2 {
                    return Err(FileSystemError::InvalidCmdArgs(
                        "usage: DELETE file".to_string(),
                    ));
                }

                let path = Path::try_from(args[1].as_str())?;

                Ok(Cmd::Delete(path))
            }
            "LIST" => {
                if args.len() != 1 {
                    return Err(FileSystemError::InvalidCmdArgs("usage: LIST".to_string()));
                }

                Ok(Cmd::List)
            }
            cmd => Err(FileSystemError::CmdNotFound(cmd.to_string())),
        }
    }
}
