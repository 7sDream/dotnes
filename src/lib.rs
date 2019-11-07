#![deny(warnings)]

pub mod header;

use header::{
    NesFileHeader,
    parser::{parse_header, ParseError},
};

#[derive(Debug)]
pub struct NesFile<'a> {
    pub header: NesFileHeader,
    pub trainer: &'a [u8],
    pub prg_rom: &'a [u8],
    pub chr_rom: &'a [u8],
    pub miscellaneous: &'a [u8],
}

const HEADER_SIZE: usize = 16;
const TRAINER_SIZE: usize = 512;

pub fn parse<'a, I: AsRef<[u8]>>(input: &I) -> Result<NesFile, ParseError> {
    let input = input.as_ref();

    if input.len() < HEADER_SIZE {
        return Err(ParseError::NotEnough);
    }

    let header = parse_header(&input[0..HEADER_SIZE])?;

    let prg_rom_start;
    let trainer= if header.has_trainer {
        if input.len() < HEADER_SIZE + TRAINER_SIZE {
            return Err(ParseError::NotEnough);
        }
        prg_rom_start = HEADER_SIZE + TRAINER_SIZE;
        &input[HEADER_SIZE..HEADER_SIZE + TRAINER_SIZE]
    } else {
        prg_rom_start = HEADER_SIZE;
        &input[HEADER_SIZE..HEADER_SIZE]
    };

    let chr_rom_start = prg_rom_start + header.prg_rom_size as usize;
    let chr_rom_end = chr_rom_start + header.chr_rom_size as usize;

    if input.len() < chr_rom_end {
        return Err(ParseError::NotEnough);
    }

    let prg_rom = &input[prg_rom_start..chr_rom_start];
    let chr_rom = &input[chr_rom_start..chr_rom_end];

    let miscellaneous = &input[chr_rom_end..];

    Ok(NesFile { header,  trainer, prg_rom, chr_rom, miscellaneous})
}
