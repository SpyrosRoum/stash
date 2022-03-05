use std::str::FromStr;

use anyhow::anyhow;

pub enum Command {
    Write(String, String),
    Exit,
    Debug,
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
            "dbg" => Ok(Self::Debug),
            "write" | "store" => {
                let k = s.next().ok_or(anyhow!("Missing key, value"))?.to_string();
                let v = s.next().ok_or(anyhow!("Missing value"))?.to_string();
                Ok(Self::Write(k, v))
            }
            _ => Err(anyhow!(format!("Unkown command `{}`", cmd))),
        }
    }
}
