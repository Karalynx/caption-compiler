
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author = "Kara")]
#[command(name = "Caption Compiler")]
#[command(about = "Compiles and describes Valve's closed captions", long_about = None)]
pub struct Arguments {
    #[clap(subcommand)]
    pub task: Task,

    #[arg(short, long, help = "Input file path")]
    pub input: PathBuf,
}

#[derive(Subcommand, Debug, Clone, Default)]
pub enum Task {
    #[clap(name = "compile", about = "Compiles to .DAT file")]
    Compile(Compile),

    #[clap(name = "describe", about = "Describes .DAT file")]
    #[default] Describe
}

#[derive(Args, Debug, Clone, Default)]
pub struct Compile {
    /// Verbose output
    #[arg(short, long, help = "Verbose output")]
    pub verbose: bool,

    /// Output folder
    #[arg(short, long, help = "Output folder")]
    pub output: Option<PathBuf>,
}