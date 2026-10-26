use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "crate-explorer", about = "Explore crate rust from crates.io")]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    Search { query: String },
    Category { category: String },
    Keyword { keyword: String },
}
