use std::io::{self, Read};

fn main() -> io::Result<()> {
    let reverse = std::env::args().any(|arg| arg == "-r");

    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;

    println!("{}", staircase::staircase(&text, reverse));

    Ok(())
}
