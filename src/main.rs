use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter your name: ");
    io::stdout().flush()?;

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();

    if name.is_empty() {
        eprintln!("Name cannot be empty.");
        std::process::exit(1);
    }

    println!("Hello, {}!", name);

    Ok(())
}
