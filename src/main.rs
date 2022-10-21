use clap::Parser;
use monirs::{
    cli::MoniCli,
    moni::{DefaultMoniPrinter, Moni},
    moni_config::{MoniConfig, MoniJsonConfig},
};
fn main() {
    //let cli = MoniCli::parse();
    //println!("{:?}", cli);
    //cli.monitaring();
    let config = MoniJsonConfig::from_file("moni.json");
    let moni = match &config {
        Ok(config) => Moni::from(config),
        Err(e) => panic!("{:?}", e),
    };
    moni.monitaring()
}
