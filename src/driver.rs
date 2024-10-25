use super::fs::{FileSystem, Cmd};
use std::io::{stdin, BufRead, BufReader};

const HELP: &str = "
A simple filesystem.

Usage: ./filesystem [OPTIONS] COMMAND

Providing no options runs the file system in interactive mode.

Command:
    FILE
        The optionally provided FILE will be processed as a series of commands.

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

        self.read_cmds(buf_reader);
    }

    pub fn run_repl(&mut self) {
        let buf_reader = BufReader::new(stdin());

        self.read_cmds(buf_reader);
    }

    fn run_cmd(&mut self, cmd: Cmd) {
        match self.fs.exec_cmd(cmd) {
            Ok(Some(output)) => {
                println!("{}", output);
            }
            Ok(None) => {},
            Err(err) => eprintln!("{err:?}"),
        }
    }

    fn read_cmds(&mut self, buf_reader: impl BufRead) {
        for line in buf_reader.lines() {
            match Cmd::try_from(line.unwrap().as_str()) {
                Ok(cmd) => self.run_cmd(cmd),
                Err(err) => eprintln!("{err:?}")
            }
        }
    }
}
