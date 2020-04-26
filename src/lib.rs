#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(warnings)]

//! Yet another library for parse NES file format.
//!
//! # Examples
//!
//! Just use the [`parse`](fn.parse.html) function with your bytes, then you are done.
//!
//! ```rust
//! use {std::fs, dotnes};
//!
//! let file = "tests/roms/cpu_tests/branch_timing_tests/1.Branch_Basics.nes";
//! let data = fs::read(file).unwrap();
//! let nes = dotnes::parse(&data).unwrap();
//! println!("NES file Header: {:#?}", nes.header);
//! println!(
//!     "PRG ROM        : {:?}...",
//!     &nes.prg_rom[0..usize::min(16, nes.prg_rom.len())]
//! );
//! println!(
//!     "CHR ROM        : {:?}...",
//!     &nes.chr_rom[0..usize::min(16, nes.chr_rom.len())]
//! );
//! println!(
//!     "Misc ROM       : {:?}...",
//!     &nes.miscellaneous_roms[0..usize::min(16, nes.miscellaneous_roms.len())]
//! );
//! ```
//!
//! See document of [`NESFile`](struct.NESFile.html) struct for parse result.

pub mod header;

pub use header::ParseHeaderError;

use header::{
    parser::parse_header,
    Header,
};

/// NES file parse result
#[derive(Debug, Clone, Hash)]
pub struct NESFile<'a> {
    /// NES file header info
    pub header: Header,
    /// Trainer data, will has 512 length when present, 0 if not
    pub trainer: &'a [u8],
    /// Main PRG-ROM data
    pub prg_rom: &'a [u8],
    /// Main CHR-ROM data, maybe 0 length
    pub chr_rom: &'a [u8],
    /// Miscellaneous ROMs, not parsed as blocks, you need split it by yourself
    /// according the header info if you want use it
    pub miscellaneous_roms: &'a [u8],
}

/// Parse failed reason
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParseError {
    /// Data is too short to be a valid nes file
    NotEnough,
    /// Error happened when parse first 16 bytes header
    InvalidHeader(ParseHeaderError),
}

impl From<ParseHeaderError> for ParseError {
    #[must_use]
    fn from(err: ParseHeaderError) -> Self {
        Self::InvalidHeader(err)
    }
}

const HEADER_SIZE: usize = 16;
const TRAINER_SIZE: usize = 512;

/// Parse your NES file content bytes to struct [`NESFile`](struct.NESFile.html).
///
/// This function will not copy any bytes, so the result has same lifetime with your bytes.
///
/// # Examples
///
/// ```rust
/// let bytes = b"some bytes"; // get your bytes from file or other place
/// let nes = dotnes::parse(&bytes);
/// ```
///
/// # Errors
///
/// When `input` is not valid NES format data, return Err([`ParseError`](enum.ParseError.html)).
pub fn parse<I: AsRef<[u8]> + ?Sized>(input: &I) -> Result<NESFile<'_>, ParseError> {
    let input = input.as_ref();

    if input.len() < HEADER_SIZE {
        return Err(ParseError::NotEnough);
    }

    let header = parse_header(&input[0..HEADER_SIZE])?;

    let prg_rom_start;
    let trainer = if header.has_trainer {
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

    let miscellaneous_roms = &input[chr_rom_end..];

    Ok(NESFile { header, trainer, prg_rom, chr_rom, miscellaneous_roms })
}
