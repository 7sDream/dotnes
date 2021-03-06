# DotNes(WIP)

![CI-badge][CI-badge]

Yet another NES file parser.

**WARNING:** I'm not an expert on NES simulator. This parser implementation is on a very early stage, it may leaking important feature/fields and contains bugs comes from my misunderstanding of wiki articles. I write this parser for my (WIP) [sau][sau-repo] project(which even not publish to GitHub for now), if you REALLY want, please use with caution.

Any bug report is welcome, But temporarily **DO NOT** accept any feature requests.

Not publish to crates.io for the same reason.

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

[CI-badge]: https://github.com/7sDream/dotnes/workflows/CI/badge.svg
[sau-repo]: https://git.7sdre.am/7sDream/sau 
[license-file]: https://github.com/7sDream/dotnes/blob/master/LICENSE
