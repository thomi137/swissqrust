
use SwissQRust::cli::*;
use clap::Parser;
fn main() {

    let cli = Cli::parse();
    println!("{:#?}", cli);
}
