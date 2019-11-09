# dotnes

Yet another .nes file parser, for my [sau][sau-repo] project.

## Usage

Just use the `dotnes::parse()` function with your bytes.

```rust
use {std::fs, dotnes};

fn main() {
    let file = "tests/roms/cpu_tests/branch_timing_tests/1.Branch_Basics.nes";
    let data = fs::read(file).unwrap();
    let nes = dotnes::parse(&data).unwrap();
    println!("NES file Header: {:#?}", nes.header);
    println!(
        "PRG ROM        : {:?}...",
        &nes.prg_rom[0..usize::min(16, nes.prg_rom.len())]
    );
    println!(
        "CHR ROM        : {:?}...",
        &nes.chr_rom[0..usize::min(16, nes.chr_rom.len())]
    );
    println!(
        "Misc ROM       : {:?}...",
        &nes.miscellaneous_roms[0..usize::min(16, nes.miscellaneous_roms.len())]
    );
}
```

Output: 

```text
NES file Header: NesFileHeader {
    prg_rom_size: 16384,
    chr_rom_size: 0,
    prg_ram_size: 8192,
    prg_nv_ram_size: 0,
    chr_ram_size: 0,
    chr_nv_ram_size: 0,
    miscellaneous_rom_count: 0,
    mapper: 0,
    sub_mapper: 0,
    is_four_screen: false,
    has_trainer: false,
    has_persistent_memory: false,
    mirroring: Horizontal,
    has_bus_conflicts: false,
    timing: NTSC,
    is_nes2: false,
    console_type: Nes,
    default_expansion_device: Unspecified,
}
PRG ROM        : [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]...
CHR ROM        : []...
Misc ROM       : []...
```

## LICENSE

Except ROM files in `tests/roms` folds, all other code are under GPLv3 License.

SEE [LICENSE][license-file].

[sau-repo]: https://git.7sdre.am/7sDream/sau 
[license-file]: https://git.7sdre.am/7sDream/dotnes/src/branch/master/LICENS
