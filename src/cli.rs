use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
#[group(id("source"), args(["file", "input"]), required=true)]
pub(crate) struct Cli {
    /// The .bf file to interpret
    #[arg(index(1), value_parser, conflicts_with("input"))]
    pub file: Option<String>,

    /// Interpret from stdin
    #[arg(short, conflicts_with("file"))]
    pub input: Option<String>,
}
