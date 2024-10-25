use endpoint_challenge::FileSystemDriver;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    FileSystemDriver::default().run_with_args(args);
}
