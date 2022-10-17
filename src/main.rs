use monirs::moni::MoniBuilder;

fn main() {
    MoniBuilder::new()
        .root("./")
        .target_extension("java")
        .exe_command("java test.java")
        .ignore_re("target")
        .build()
        .monitaring();
}
