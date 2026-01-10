use clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
author="Thomas Prosser <thomas@prosser.ch>",
version,
about = "Swiss Payment Slips in Rust",
long_about = None)]
#[command(subcommand_precedence_over_arg = true)]
pub struct Cli {
    #[arg(short, long)]
    pub iban: String,


}
