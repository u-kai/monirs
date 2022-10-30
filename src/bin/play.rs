use monirs::parts::filesearcher::FileSearcherBuilder;

fn main() {
    let filesearcher = FileSearcherBuilder::new()
        .root("./")
        .target_extension("rs")
        .build();
    println!("{:#?}", filesearcher.get_all_files());
}
