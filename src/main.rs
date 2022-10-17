use monirs::moni::MoniBuilder;

fn main() {
    MoniBuilder::new()
        .root("../../")
        .ignore_re("target")
        .ignore_extension("rs")
        .build()
        .monitaring(0, 100000000);
}
