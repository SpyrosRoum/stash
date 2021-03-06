mod command;
mod custom_prompt;

use std::{env, fs::OpenOptions, process::exit, str::FromStr};

use {
    anyhow::{Context, Result},
    reedline::{Reedline, Signal},
    stash::{Section, Storage},
};

use {command::Command, custom_prompt::CustomPrompt};

const EXIT_MSG: &str = "Byee";

fn main() -> Result<()> {
    let file = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: `stash <file>`");
        exit(1);
    });
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file)
        .with_context(|| format!("Failed to open `{}`", file))?;

    let mut storage = Storage::new(file);

    let mut rl = Reedline::create().context("Something went wrong")?;
    let prompt = CustomPrompt;

    loop {
        let sig = rl.read_line(&prompt).context("Failed to read input")?;
        match sig {
            Signal::Success(buff) => {
                if let Err(e) = handle_line(&mut storage, &buff) {
                    eprintln!("{}", e);
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

fn handle_line(storage: &mut Storage, msg: &str) -> Result<()> {
    let cmd = Command::from_str(msg)?;
    match cmd {
        Command::Debug => {
            println!("{:#?}", storage);
        }
        Command::Exit => {
            println!("{}", EXIT_MSG);
            exit(0);
        }
        Command::Write(key, value) => {
            let section = Section::new(key, value);
            storage.write(section)?;
        }
        Command::Dump => {
            storage.dump()?;
        }
        Command::None => {}
    }

    Ok(())
}
