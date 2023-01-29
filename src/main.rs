use std::process::Command;
use std::io::Write;
use std::io;

mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    print!("Enter show name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    if input == "break" {
        std::process::exit(0);
    }

    // Get shows from lib.rs
    let (subtitle, urls) = lib::main(input.to_string()).unwrap();
    print!("{:?}", subtitle);

    let mpv_command = Command::new("mpv")
        .args([
            format!("{}", urls[0].as_str()),
        ])
        .spawn();

    Ok(())
}
