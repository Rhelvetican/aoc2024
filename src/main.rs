use ansi_term::enable_ansi_support;
use aoc2024::{
    cli::AocCli,
    map_solution,
    solutions::*,
    utils::{Error, Result},
};
use clap::Parser;

fn main() -> Result<()> {
    if let Err(e) = enable_ansi_support() {
        println!("{e}");
    }

    let cli = match AocCli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            println!("{}", e.render().ansi());
            return Ok(());
        }
    };

    println!("Solution for Day {}:", cli.day);
    map_solution! {cli :
        1 => AocDayOneSolution,
        2 => AocDayTwoSolution,
        3 => AocDayThreeSolution,
        4 => AocDayFourSolution,
        5 => AocDayFiveSolution,
        6 => AocDaySixSolution,
        7 => AocDaySevenSolution,
        8 => AocDayEightSolution,
        9 => AocDayNineSolution
    };

    Ok(())
}
