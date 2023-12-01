use std::{error::Error, path::Path, io::Read, process};

use clap::Parser;

use interpreter::Interpreter;

mod interpreter;
mod cli;

fn main() {
    let cli = cli::Cli::parse();

    let source = if let Some(file) = &cli.file {
        std::fs::read_to_string(file).map_err(|e| {
            println!("\x1b[1;31mError\x1b[0m\t{} [{}]", e, file);
            process::exit(e.raw_os_error().unwrap_or(1));
        }).unwrap()
    } else if let Some(input) = cli.input {
        input
    } else { unreachable!() };

    let mut interpreter = Interpreter::new(source);
    interpreter.interpret();
}

