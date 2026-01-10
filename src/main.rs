use crate::cli::Cli;

mod cli;
use clap::Parser;
fn main() {

    let cli = Cli::parse();
    println!("{:#?}", cli);
}
