use clap::Parser;
use monirs::cli::MoniCli;
fn main() {
    let cli = MoniCli::parse();
    cli.monitaring();
}
