use std::io::{self, Write};

mod message;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer)?;

    let mut stdout = io::stdout().lock();

    stdout.write_all(buffer.as_bytes())?;

    Ok(())
}
