use std::{
    io::Read,
    convert::{From, TryFrom},
};

use nom::{
    IResult as NomResult,
    Slice as NomSlice,
    bytes::complete as NomBytes,
    sequence as NomSeq,
    multi as NomMulti,
    bits as NomConvert,
    bits::complete as NomBits,
    number::complete as NomNum,
    Err as NomErr,
    error::ErrorKind as NomErrorKind,
};

use num_enum::TryFromPrimitive;

#[derive(Debug)]
pub enum ParseError {
    IoError(std::io::Error),
    DataInvalid(String),
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError(err)
    }
}

impl<T> From<NomErr<(T, NomErrorKind)>> for ParseError {
    fn from(err: NomErr<(T, NomErrorKind)>) -> Self {
        ParseError::DataInvalid(match err {
            NomErr::Incomplete(_) => "There was not enough data".to_string(),
            NomErr::Failure(e) | NomErr::Error(e) => e.1.description().to_string(),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum Timing {
    NTSC = 0x0,
    PAL = 0x1,
    MultipleRegion = 0x2,
    Dendy = 0x3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum VsPPUType {
    RP2C03B = 0x0,
    RP2C03G = 0x1,
    RP2C040001 = 0x2,
    RP2C040002 = 0x3,
    RP2C040003 = 0x4,
    RP2C040004 = 0x5,
    RC2C03B = 0x6,
    RC2C03C = 0x7,
    RC2C0501 = 0x8,
    RC2C0502 = 0x9,
    RC2C0503 = 0xA,
    RC2C0504 = 0xB,
    RC2C0505 = 0xC,
    ReservedD = 0xD,
    ReservedE = 0xE,
    ReservedF= 0xF,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum VsHardwareType {
    UniSystemNormal = 0x0,
    UniSystemRBIBaseballProtection = 0x1,
    UniSystemTKOBoxingProtection = 0x2,
    UniSystemSuperXeviousProtection= 0x3,
    UniSystemVsIceClimberJapanProtection = 0x4,
    DualSystemNormal = 0x5,
    DualSystemRaidOnBungelingBayProtection = 0x6,
    Reserved7 = 0x7,
    Reserved8 = 0x8,
    Reserved9 = 0x9,
    ReservedA = 0xA,
    ReservedB = 0xB,
    ReservedC = 0xC,
    ReservedD = 0xD,
    ReservedE = 0xE,
    ReservedF = 0xF,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VsInfo {
    ppu_type: VsPPUType,
    hardware_type: VsHardwareType,
}

impl Default for VsInfo {
    fn default() -> Self {
        Self {
            ppu_type: VsPPUType::RP2C03B,
            hardware_type: VsHardwareType::UniSystemNormal,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum ExtendedConsoleType {
    Regular = 0x0,
    Vs = 0x1,
    Pc10 = 0x2,
    RegularWithDecimal = 0x3,
    VT01WithMonochrome = 0x4,
    VT01WithRedCyanSTN = 0x5,
    VT02 = 0x6,
    VT03 = 0x7,
    VT09 = 0x8,
    VT32 = 0x9,
    VT369 = 0xA,
    ReversedB = 0xB,
    ReversedC = 0xC,
    ReversedD = 0xD,
    ReversedE = 0xE,
    ReversedF = 0xF,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConsoleType {
    Nes,
    Vs(VsInfo),
    Pc10,
    Extend(ExtendedConsoleType),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NesFileHeader {
    prg_rom_size: u16,
    chr_rom_size: u16,
    prg_ram_size: u32,
    prg_nv_ram_size: u32,
    chr_ram_size: u32,
    chr_nv_ram_size: u32,
    miscellaneous_rom_count: u8,
    mapper: u16,
    sub_mapper: u8,
    is_four_screen: bool,
    has_trainer: bool,
    has_persistent_memory: bool,
    mirroring: Mirroring,
    timing: Timing,
    is_nes2: bool,
    console_type: ConsoleType,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NesFile {
    header: NesFileHeader
}

fn bits_tuple<I, O, L>(l: L) -> impl Fn(I) -> NomResult<I, O>
    where
        I: NomSlice<std::ops::RangeFrom<usize>> + Clone,
        L: NomSeq::Tuple<(I, usize), O, ((I, usize), NomErrorKind)> {
    NomConvert::bits(NomSeq::tuple::<_, _, ((I, usize), NomErrorKind), _>(l))
}

fn parse_flag6(input: &[u8]) -> NomResult<&[u8], (u8, u8, u8, u8, u8)> {
    let (input, (mapper_lo, flags)) = bits_tuple((
        NomBits::take(4u8),
        NomMulti::count(NomBits::take(1u8), 4)
    ))(input)?;

    Ok((input, (mapper_lo, flags[0], flags[1], flags[2], flags[3])))
}

fn parse_flag7(input: &[u8]) -> NomResult<&[u8], (u8, u8, u8)> {
    let (input, (mapper_mid, nes2, console_type)) = bits_tuple((
        NomBits::take(4u8),
        NomBits::take(2u8),
        NomBits::take(2u8),
    ))(input)?;
    Ok((input, (mapper_mid, nes2, console_type)))
}

fn parse_flag8(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((
        NomBits::take(4u8),
        NomBits::take(4u8),
    ))(input)
}

fn parse_flag9(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((
        NomBits::take(4u8),
        NomBits::take(4u8),
    ))(input)
}

fn parse_flag10(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((
        NomBits::take(4u8),
        NomBits::take(4u8),
    ))(input)
}

fn parse_flag11(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((
        NomBits::take(4u8),
        NomBits::take(4u8),
    ))(input)
}

fn parse_flag12(input: &[u8]) -> NomResult<&[u8], u8> {
    let (input, (_unused, timing)): (_, (u8, _ )) = bits_tuple((
        NomBits::take(6u8),
        NomBits::take(2u8),
    ))(input)?;
    Ok((input, timing))
}

fn parse_flag13(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((
        NomBits::take(4u8),
        NomBits::take(4u8),
    ))(input)
}

fn parse_flag14(input: &[u8]) -> NomResult<&[u8], u8> {
    let (input, (_unused, timing)): (_, (u8, _ )) = bits_tuple((
        NomBits::take(6u8),
        NomBits::take(2u8),
    ))(input)?;
    Ok((input, timing))
}

fn parse_header(input: &[u8]) -> NomResult<&[u8], NesFileHeader> {
    let (input, (_, prg_rom_size_lo, chr_rom_size_lo)) = NomSeq::tuple((NomBytes::tag("NES\x1A"), NomNum::le_u8, NomNum::le_u8))(input)?;
    let mut prg_rom_size = prg_rom_size_lo as u16;
    let mut chr_rom_size = chr_rom_size_lo as u16;
    let (input, (mapper_lo, f, t, b, m)) = parse_flag6(input)?;
    let (input, (mut mapper_mid, nes2, console_type)) = parse_flag7(input)?;
    let is_nes2 = nes2 == 0b10;
    let mut console = match console_type {
        0 => ConsoleType::Nes,
        1 => ConsoleType::Vs(VsInfo::default()),
        2 => ConsoleType::Pc10,
        3 => if is_nes2 {
            ConsoleType::Extend(ExtendedConsoleType::Regular)
        } else {
            // FIXME: Can iNES 1.0 format's console_type bits be 0b11?
            ConsoleType::Vs(VsInfo::default())
        },
        _ => unreachable!("console type must in 0 - 3"),
    };
    let mut sub_mapper = 0;
    let mut prg_ram_size: u32 = 0;
    let mut prg_nv_ram_size: u32 = 0;
    let mut chr_ram_size: u32 = 0;
    let mut chr_nv_ram_size: u32 = 0;
    let mut timing = Timing::NTSC;
    let mut miscellaneous_rom_count = 0;
    if is_nes2 {
        let (input, (sub_mapper_actual, mapper_hi)) = parse_flag8(input)?;
        sub_mapper = sub_mapper_actual;
        mapper_mid |= mapper_hi << 4;

        let (input, (prg_rom_size_hi, chr_rom_size_hi)) = parse_flag9(input)?;
        prg_rom_size |= (prg_rom_size_hi as u16) << 8;
        chr_rom_size |= (chr_rom_size_hi as u16) << 8;

        let (input, (prg_ram_shift, prg_nv_ram_shift)) = parse_flag10(input)?;
        if prg_ram_shift != 0 {
            prg_ram_size = 64u32 << prg_ram_shift as u32;
        }
        if prg_nv_ram_shift != 0 {
            prg_nv_ram_size = 64u32 << prg_nv_ram_shift as u32;
        }

        let (input, (chr_ram_shift, chr_nv_ram_shift)) = parse_flag11(input)?;
        if chr_ram_shift != 0 {
            chr_ram_size = 64u32 << chr_ram_shift as u32;
        }
        if chr_nv_ram_shift != 0 {
            chr_nv_ram_size = 64u32 << chr_nv_ram_shift as u32;
        }

        let (input, timing_actual) = parse_flag12(input)?;
        timing = Timing::try_from(timing_actual).unwrap();

        let (input, (a, b)) = parse_flag13(input)?;
        if let ConsoleType::Vs(ref mut info) = console {
            info.hardware_type = VsHardwareType::try_from(a).unwrap();
            info.ppu_type = VsPPUType::try_from(b).unwrap();
        } else if let ConsoleType::Extend(ref mut extend) = console {
            *extend = ExtendedConsoleType::try_from(b).unwrap();
        }

        let (input, miscellaneous_rom_count_actual) = parse_flag14(input)?;
        miscellaneous_rom_count = miscellaneous_rom_count_actual;
    } else {

    }
    Ok((input, NesFileHeader {
        prg_rom_size,
        chr_rom_size,
        prg_ram_size,
        prg_nv_ram_size,
        chr_ram_size,
        chr_nv_ram_size,
        miscellaneous_rom_count,
        mapper: mapper_lo as u16 | ((mapper_mid as u16) << 4),
        sub_mapper,
        is_four_screen: f == 1,
        has_trainer: t == 1,
        has_persistent_memory: b == 1,
        mirroring: if m == 1 { Mirroring::Vertical } else { Mirroring::Horizontal },
        timing,
        is_nes2,
        console_type: console,
    }))
}

fn parse_all(input: &[u8]) -> NomResult<&[u8], NesFile> {
    let (input, header) = parse_header(input)?;
    Ok((input, NesFile { header }))
}

pub fn parse<R: Read>(reader: &mut R) -> Result<NesFile, ParseError> {
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    let (_, nes_file) = parse_all(&data)?;
    Ok(nes_file)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn test_parse_success() {
        let mut data = BufReader::new("NES\x1A\x12\x34\x5C\x69\x77\x77\x07\x70\x01\x36\x03".as_bytes());
        let result = parse(&mut data).unwrap();
        assert_eq!(result.header.prg_rom_size, 0x712);
        assert_eq!(result.header.chr_rom_size, 0x734);
        assert_eq!(result.header.prg_ram_size, 0);
        assert_eq!(result.header.prg_nv_ram_size, 8192);
        assert_eq!(result.header.chr_ram_size, 8192);
        assert_eq!(result.header.chr_nv_ram_size, 0);
        assert_eq!(result.header.miscellaneous_rom_count, 3);
        assert_eq!(result.header.mapper, 0x765);
        assert_eq!(result.header.sub_mapper, 0x7);
        assert!(result.header.is_four_screen);
        assert!(result.header.has_trainer);
        assert_eq!(result.header.has_persistent_memory, false);
        assert_eq!(result.header.mirroring, Mirroring::Horizontal);
        assert_eq!(result.header.timing, Timing::PAL);
        assert_eq!(result.header.console_type, ConsoleType::Vs(VsInfo {
            ppu_type: VsPPUType::RC2C03B,
            hardware_type: VsHardwareType::UniSystemSuperXeviousProtection
        }));
    }

    #[test]
    fn test_parse_eof() {
        let mut data = BufReader::new("NES\x1A".as_bytes());
        let result = parse(&mut data);
        assert_eq!(match result {
            Err(ParseError::DataInvalid(ref s)) => s,
            Err(_) => panic!("return an incorrect parse error"),
            Ok(_) => panic!("parse success on error data"),
        }, "End of file");
    }
}
