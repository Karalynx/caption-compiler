
use clap::Parser;

use caption_compiler::cli::{self, Arguments, Task};

fn main() {
    let args = Arguments::parse();
    match args.task {
        Task::Compile(compile) => {
            if let Err(err) = cli::compile(args.input, compile) {
                println!("{err}")
            }
        }
        Task::Describe => {
            if let Err(err) = cli::describe(args.input) {
                println!("{err}")
            }
        }
    };
}