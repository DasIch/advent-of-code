extern crate anyhow;
extern crate clap;
extern crate md5;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap::{ArgEnum, Parser};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(arg_enum)]
    command: Command,

    #[clap(default_value = "-")]
    input: PathBuf,
}

#[derive(ArgEnum, Clone, Copy, Debug)]
enum Command {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
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

    let command = match &args.command {
        Command::Day01 => day01::main,
        Command::Day02 => day02::main,
        Command::Day03 => day03::main,
        Command::Day04 => day04::main,
        Command::Day05 => day05::main,
        Command::Day06 => day06::main,
        Command::Day07 => day07::main,
        Command::Day08 => day08::main,
        Command::Day09 => day09::main,
        Command::Day10 => day10::main,
        Command::Day11 => day11::main,
        Command::Day12 => day12::main,
        Command::Day13 => day13::main,
        Command::Day14 => day14::main,
        Command::Day15 => day15::main,
        Command::Day16 => day16::main,
        Command::Day17 => day17::main,
        Command::Day18 => day18::main,
        Command::Day19 => day19::main,
        Command::Day20 => day20::main,
        Command::Day21 => day21::main,
        Command::Day22 => day22::main,
        Command::Day23 => day23::main,
    };
    command(input.as_str())?;

    Ok(())
}
