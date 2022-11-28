use crossterm::style::{Color, SetForegroundColor};
use std::io::Error;
use std::io::{self, Write};

fn main() -> Result<(), Error> {
    let mut stdout = io::stdout();
    writeln!(
        stdout,
        "{}Riced CLI v0.0.1{}",
        SetForegroundColor(Color::Yellow),
        SetForegroundColor(Color::Reset)
    )?;
    Ok(())
}
