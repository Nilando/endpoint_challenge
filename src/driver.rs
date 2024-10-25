use super::fs::{FileSystem, Cmd};
use std::io::{stdin, stdout, Read, BufRead, BufReader, BufWriter, Write};

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
    writer: Box<dyn Write>,
}

impl Default for FileSystemDriver {
    fn default() -> Self {
        Self::new(Box::new(BufWriter::new(stdout().lock())))
    }
}

impl FileSystemDriver {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self {
            fs: FileSystem::new(),
            writer,
        }
    }

    pub fn run_with_args(&mut self, args: Vec<String>) {
        match args.len() {
            1 => self.run_repl(),
            2 => {
                let arg = &args[1];
                if arg == "-h" || arg == "--help" {
                    writeln!(self.writer, "{}", HELP);
                }

                let filepath = arg.clone();
                self.run_file(filepath);
            }
            _ => {
                writeln!(self.writer, "{}", USAGE).unwrap();
            }
        }

        self.writer.flush();
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
                    writeln!(self.writer, "{}", line);
                    self.exec_cmd(cmd)
                }
                Err(err) => {
                    writeln!(self.writer, "{}", err);
                }
            };

            self.writer.flush();
        }
    }

    fn exec_cmd(&mut self, cmd: Cmd) {
        match self.fs.exec_cmd(cmd.clone()) {
            Ok(ok) => {
                if let Some(output) = ok {
                    writeln!(self.writer, "{}", output);
                }
            }
            Err(err) => {
                match cmd {
                    Cmd::Delete(path) => {
                        writeln!(self.writer, "Cannot delete {} - {}", path, err);
                    }
                    Cmd::Move { src, dest } => {
                        writeln!(self.writer, "Cannot move {} {} - {}", src, dest, err);
                    }
                    Cmd::Create(path) => {
                        writeln!(self.writer, "Cannot create {} - {}", path, err);
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    fn test_example(x: usize) {
        let out_file = format!("./tests/test_example_{}.out", x);
        let input_file = format!("./tests/test_example_{}.input", x);
        let expect_file = format!("./tests/test_example_{}.expect", x);

        {
            let file = File::create(out_file.clone()).unwrap();
            let writer = Box::new(BufWriter::new(file));
            let mut driver = FileSystemDriver::new(writer);

            driver.run_file(input_file.to_string());
        }

        let mut result = File::open(out_file).unwrap();
        let mut expect = File::open(expect_file).unwrap();

        let mut b1 = Vec::new();
        let mut b2 = Vec::new();
        result.read_to_end(&mut b1);
        expect.read_to_end(&mut b2);

        assert_eq!(b1, b2);
    }

    #[test]
    fn test_example_1() {
        test_example(1);
    }

    #[test]
    fn test_example_2() {
        test_example(2);
    }

    #[test]
    fn test_example_3() {
        test_example(3);
    }

    #[test]
    fn test_example_4() {
        test_example(4);
    }
}
