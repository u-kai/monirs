use monirs::moni::MoniBuilder;

fn main() {
    let moni = MoniBuilder::new()
        .root("./")
        .ignore_re("target")
        .build()
        .monitaring(0, 100000000);
}
