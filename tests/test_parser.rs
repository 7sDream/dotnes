use {dotnes, std::fs, walkdir::WalkDir};

#[test]
fn parse_all_valid_roms() {
    for file in WalkDir::new("tests/roms")
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension().unwrap_or_default() == "nes"
        })
    {
        let data = fs::read(file.path()).unwrap();
        let nes_file = dotnes::parse(&data).unwrap();
        println!("{:#?}", nes_file.header);
    }
}
