use std::str::FromStr;

use anyhow::anyhow;

pub enum Command {
    Exit,
    None,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();

        let cmd = s.next();
        if cmd.is_none() {
            return Ok(Self::None);
        }
        let cmd = cmd.unwrap();

        match cmd {
            "exit" => Ok(Self::Exit),
            _ => Err(anyhow!(format!("Unkown command `{}`", cmd))),
        }
    }
}
