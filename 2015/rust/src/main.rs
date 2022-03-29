extern crate anyhow;
extern crate clap;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap::{ArgEnum, Parser};

mod day01;
mod day02;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(arg_enum)]
    command: Commands,

    #[clap(default_value = "-")]
    input: PathBuf,
}

#[derive(ArgEnum, Clone, Copy, Debug)]
enum Commands {
    Day01,
    Day02,
}

/// Reads the file at `path` into a String. If the `path` is `-`, stdin is read
/// instead of the file at path `-`.
fn read_input(path: &Path) -> std::io::Result<String> {
    let mut buffer = String::new();
    if let Some("-") = path.to_str() {
        std::io::stdin().read_to_string(&mut buffer)?;
    } else {
        let mut f = File::open(path)?;
        f.read_to_string(&mut buffer)?;
    }
    Ok(buffer)
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let input = read_input(args.input.as_path())?;

    match &args.command {
        Commands::Day01 => {
            day01::main(input.as_str())?;
        }
        Commands::Day02 => {
            day02::main(input.as_str())?;
        }
    }

    Ok(())
}
