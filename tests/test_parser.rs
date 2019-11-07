use std::path::Path;

use dotnes;

#[test]
fn readme_example_works() {
    let file = "tests/rom/1.Branch_Basics.nes";
    let data = std::fs::read(file).unwrap();
    let nes = dotnes::parse(&data).unwrap();
    println!("NES file Header: {:#?}", nes.header);
    println!("PRG ROM        : {:?}...", &nes.prg_rom[0..usize::min(16, nes.prg_rom.len())]);
    println!("CHR ROM        : {:?}...", &nes.chr_rom[0..usize::min(16, nes.chr_rom.len())]);
    println!("Misc ROM       : {:?}...", &nes.miscellaneous[0..usize::min(10, nes.miscellaneous.len())]);
}


#[test]
fn parse_all_valid_roms() {
    for file in Path::new("tests/rom").read_dir().unwrap() {
        let file = file.unwrap();
        if file.path().extension().unwrap_or_default() != "nes" {
            continue;
        }
        let data = std::fs::read(file.path()).unwrap();
        assert!(dotnes::parse(&data).is_ok());
    }
}
