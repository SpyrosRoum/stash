mod command;
mod custom_prompt;

use std::{process::exit, str::FromStr};

use {
    anyhow::{Context, Result},
    reedline::{Reedline, Signal},
};

use {command::Command, custom_prompt::CustomPrompt};

const EXIT_MSG: &str = "Byee";

fn main() -> Result<()> {
    let mut rl = Reedline::create().context("Something went wrong")?;
    let prompt = CustomPrompt;

    loop {
        let sig = rl.read_line(&prompt).context("Failed to read input")?;
        match sig {
            Signal::Success(buff) => {
                if let Err(e) = handle_line(&buff) {
                    eprintln!("{}", e.to_string());
                }
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("{}", EXIT_MSG);
                break;
            }
            Signal::CtrlL => rl.clear_screen().context("Failed to clear screen")?,
        }
    }

    Ok(())
}

fn handle_line(msg: &str) -> Result<()> {
    let cmd = Command::from_str(msg)?;
    match cmd {
        Command::Exit => {
            println!("{}", EXIT_MSG);
            exit(0);
        }
        Command::None => {}
    }

    Ok(())
}
