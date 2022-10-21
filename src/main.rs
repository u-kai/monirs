use monirs::{
    moni::DefaultMoniPrinter,
    moni_config::{MoniConfig, MoniJsonConfig},
};
fn main() {
    MoniJsonConfig::from_file("moni.json")
        .unwrap()
        .to_moni(DefaultMoniPrinter::default())
        .monitaring()
}
