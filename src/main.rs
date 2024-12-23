use std::env;
use std::fs;

use colored::Colorize;

const HISTFILE: &str = "HISTFILE";

fn main() -> Result<(), hmanic::Error> {
    let hist_path = env::var(HISTFILE)?;
    println!("The current shell's history file can be found at `{}`", hist_path.bright_cyan());

    let contents = fs::read_to_string(hist_path)?;
    println!("{}", contents);

    Ok(())
}
