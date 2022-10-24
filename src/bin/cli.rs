use clap::Parser;
use monirs::cli;
// use cli
fn main() {
    cli::MoniCli::parse().monitaring()
}
