use std::{error::Error, path::Path, io::Read};

use clap::Parser;

use interpreter::Interpreter;

mod interpreter;
mod cli;

fn main() {
    let cli = cli::Cli::parse();

    let source = if let Some(file) = cli.file {
        read_file(file).unwrap()
    } else if let Some(input) = cli.input {
        input
    } else { unreachable!() };

    let mut interpreter = Interpreter::new(source);
    interpreter.interpret();
}

fn read_file(path: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
