use super::fs::{FileSystem, Cmd};
use std::io::{stdin, BufRead, BufReader};

const HELP: &str = "
A simple filesystem.

Usage: ./filesystem [OPTIONS] COMMAND

Providing no command runs the file system in interactive mode.

Command:
    FILE
        The FILE will be processed as a series of commands.

Options:
    -h, --help
        Print Help
";

const USAGE: &str = "
Error: invalid number of args

Usage: ./filesystem [OPTIONS]

For more information, try '--help'.
";

pub struct FileSystemDriver {
    fs: FileSystem,

}

impl FileSystemDriver {
    pub fn new() -> Self {
        Self {
            fs: FileSystem::new()
        }
    }

    pub fn run_with_args(&mut self, args: Vec<String>) {
        match args.len() {
            1 => self.run_repl(),
            2 => {
                let arg = &args[1];
                if arg == "-h" || arg == "--help" {
                    println!("{}", HELP);
                }

                let filepath = arg.clone();
                self.run_file(filepath);
            }
            _ => eprintln!("{}", USAGE),
        }
    }

    pub fn run_file(&mut self, filepath: String) {
        let file = std::fs::File::open(filepath).unwrap();
        let buf_reader = BufReader::new(file);

        self.cmd_loop(buf_reader);
    }

    pub fn run_repl(&mut self) {
        let buf_reader = BufReader::new(stdin());

        self.cmd_loop(buf_reader);
    }

    fn cmd_loop(&mut self, buf_reader: impl BufRead) {
        for res in buf_reader.lines() {
            let line = res.unwrap();
            if line.is_empty() { 
                continue
            }

            match Cmd::try_from(line.as_str()) {
                Ok(cmd) => {
                    println!("{}", line);
                    self.exec_cmd(cmd)
                }
                Err(err) => eprintln!("{}", err)
            }
        }
    }

    fn exec_cmd(&mut self, cmd: Cmd) {
        match self.fs.exec_cmd(cmd.clone()) {
            Ok(ok) => {
                if let Some(output) = ok {
                    println!("{}", output);
                }
            }
            Err(err) => {
                match cmd {
                    Cmd::Delete(path) => {
                        eprintln!("Cannot delete {} - {}", path, err)
                    }
                    Cmd::Move { src, dest } => {
                        eprintln!("Cannot move {} {} - {}", src, dest, err)
                    }
                    Cmd::Create(path) => {
                        eprintln!("Cannot create {} - {}", path, err)
                    }
                    _ => {}
                }
            }
        }
    }
}
