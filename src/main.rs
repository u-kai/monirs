use monirs::moni::MoniBuilder;

fn main() {
    MoniBuilder::new()
        .root("./")
        .target_extension("py")
        .exe_command("python test.py")
        .ignore_re("target")
        .build()
        .monitaring();
}
