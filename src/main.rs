use monirs::{
    moni::DefaultMoniPrinter,
    moni_config::{MoniConfig, MoniJsonConfig},
};

fn main() {
    let moni = MoniJsonConfig::from_file("moni.json").unwrap();
    moni.to_moni(DefaultMoniPrinter::default()).monitaring();
}
