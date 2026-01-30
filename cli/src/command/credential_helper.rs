use std::io::{self, BufRead, Write};

use anyhow::Result;

use crate::keychain;

pub fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();

    // Read all input from git
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        input.push_str(&line);
        input.push('\n');
    }

    // Parse the input to check if this is a "get" request
    // Git sends: protocol=https\nhost=example.com\n
    let mut protocol = None;
    let mut host = None;

    for line in input.lines() {
        if let Some((key, value)) = line.split_once('=') {
            match key {
                "protocol" => protocol = Some(value.to_string()),
                "host" => host = Some(value.to_string()),
                _ => {}
            }
        }
    }

    // Only respond if we have a token stored
    if let Some(token) = keychain::get_token()? {
        // Output credentials in git credential format
        let mut stdout = io::stdout().lock();

        if let Some(protocol) = protocol {
            writeln!(stdout, "protocol={}", protocol)?;
        }
        if let Some(host) = host {
            writeln!(stdout, "host={}", host)?;
        }

        // Use a placeholder username since Basic auth requires both
        writeln!(stdout, "username=x-token")?;
        writeln!(stdout, "password={}", token)?;
        writeln!(stdout)?;
    }

    Ok(())
}
