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

    #[arg(short, long)]
    pub text: String,

    #[arg(short = 'c', long)]
    pub creditor_name: String,

    #[arg(short = '1', long)]
    creditor_address_line1: String,

    #[arg(short = '2', long)]
    creditor_address_line2: String,

    #[arg(short = 'a', long)]
    pub creditor_street_name: String,

    #[arg(short = 'n', long)]
    pub creditor_house_number: String,

    #[arg(short= 'p', long)]
    pub creditor_postal_code: i32,

    #[arg(short = 's', long)]
    creditor_city: String,

    #[arg(short = 'l', long)]
    creditor_country: String,

    #[arg(short = 'm', long)]
    amount_payable: f64,

}
