use std::{
    io::Read,
    convert::From,
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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    Reserved = 0xD,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VsHardwareType {
    UniSystemNormal = 0x0,
    UniSystemRBIBaseballProtection = 0x1,
    UniSystemTKOBoxingProtection = 0x2,
    UniSystemSuperXeviousProtection= 0x3,
    UniSystemVsIceClimberJapanProtection = 0x4,
    DualSystemNormal = 0x5,
    DualSystemRaidOnBungelingBayProtection = 0x6,
    Reserved = 0x7,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VsInfo {
    ppu_type: VsPPUType,
    hardware_type: VsHardwareType,
}

impl Default for VsInfo {
    fn default() -> Self {
        Self {
            ppu_type: VsPPUType::Reserved,
            hardware_type: VsHardwareType::Reserved,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    Reversed = 0xB,
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
    prg_size: u16,
    chr_size: u16,
    mapper: u16,
    sub_mapper: u8,
    is_four_screen: bool,
    has_trainer: bool,
    has_persistent_memory: bool,
    mirroring: Mirroring,
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

fn parse_header(input: &[u8]) -> NomResult<&[u8], NesFileHeader> {
    let (input, (_, prg_size_lo, chr_size_lo)) = NomSeq::tuple((NomBytes::tag("NES\x1A"), NomNum::le_u8, NomNum::le_u8))(input)?;
    let mut prg_size = prg_size_lo as u16;
    let mut chr_size = chr_size_lo as u16;
    let (input, (mapper_lo, f, t, b, m)) = parse_flag6(input)?;
    let (input, (mut mapper_mid, nes2, console_type)) = parse_flag7(input)?;
    let is_nes2 = nes2 == 0b10;
    let console = match console_type {
        0 => ConsoleType::Nes,
        1 => ConsoleType::Vs(VsInfo::default()),
        2 => ConsoleType::Pc10,
        3 => if is_nes2 {
            ConsoleType::Extend(ExtendedConsoleType::Reversed)
        } else {
            ConsoleType::Nes // FIXME: Can iNES 1.0 format's console_type bits be 0b11?
        },
        _ => unreachable!("console type must in 0 - 3"),
    };
    let mut sub_mapper = 0;
    if is_nes2 {
        let (input, (sub_mapper_actual, mapper_hi)) = parse_flag8(input)?;
        sub_mapper = sub_mapper_actual;
        mapper_mid |= mapper_hi << 4;
        let (input, (prg_size_hi, chr_size_hi)) = parse_flag9(input)?;
        prg_size |= (prg_size_hi as u16) << 8;
        chr_size |= (chr_size_hi as u16) << 8;
        // TODO stop at here
    } else {

    }
    Ok((input, NesFileHeader {
        prg_size, chr_size,
        mapper: mapper_lo as u16 | ((mapper_mid as u16) << 4),
        sub_mapper,
        is_four_screen: f == 1,
        has_trainer: t == 1,
        has_persistent_memory: b == 1,
        mirroring: if m == 1 { Mirroring::Vertical } else { Mirroring::Horizontal },
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
    fn test_parse_tag() {
        let mut data = BufReader::new("NES\x1A\x12\x34\x5C\x68\x77\x77".as_bytes());
        let result = parse(&mut data).unwrap();
        assert_eq!(result.header.prg_size, 0x712);
        assert_eq!(result.header.chr_size, 0x734);
        assert_eq!(result.header.mapper, 0x765);
        assert_eq!(result.header.sub_mapper, 0x7);
        assert!(result.header.is_four_screen);
        assert!(result.header.has_trainer);
        assert_eq!(result.header.has_persistent_memory, false);
        assert_eq!(result.header.mirroring, Mirroring::Horizontal);

        let mut data = BufReader::new("NES\x1A".as_bytes());
        let result = parse(&mut data);
        assert_eq!(match result {
            Err(ParseError::DataInvalid(ref s)) => s,
            Err(_) => panic!("return an incorrect parse error"),
            Ok(_) => panic!("parse success on error data"),
        }, "End of file");
    }
}