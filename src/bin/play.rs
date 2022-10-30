use monirs::parts::filesearcher::FileSearcherBuilder;

fn main() {
    let mut filesearcher = FileSearcherBuilder::new()
        .root("./")
        .target_extension("rs")
        .build();
    println!("{:#?}", filesearcher.get_all_files());
}
