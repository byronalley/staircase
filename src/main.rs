use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;

    println!("{}", staircase::staircase(text.as_str()));

    Ok(())
}
