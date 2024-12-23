use std::env;

const HISTFILE: &str = "HISTFILE";

fn main() -> Result<(), env::VarError> {
    let hist_path = env::var(HISTFILE)?;
    println!("The current shell's history file can be found at `{}`", hist_path);

    Ok(())
}
