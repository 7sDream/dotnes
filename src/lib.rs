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

pub struct NesFileHeader {
    prg_size: u8,
    chr_size: u8,
    mapper: u16,
    is_four_screen: bool,
    has_trainer: bool,
    has_persistent_memory: bool,
    mirroring: Mirroring
}

pub struct NesFile {
    header: NesFileHeader
}

fn bits_tuple<I, O, L>(l: L) -> impl Fn(I) -> NomResult<I, O>
    where
        I: NomSlice<std::ops::RangeFrom<usize>> + Clone,
        L: NomSeq::Tuple<(I, usize), O, ((I, usize), NomErrorKind)> {
    NomConvert::bits(NomSeq::tuple::<_, _, ((I, usize), NomErrorKind), _>(l))
}

fn parse_flag6(input: &[u8]) -> NomResult<&[u8], (u8, bool, bool, bool, bool)> {
    let (input, (mapper_lo, flags)): (_, (_, Vec<u8>)) = bits_tuple((
        NomBits::take(4u8),
        NomMulti::count(NomBits::take(1u8), 4)
    ))(input)?;

    Ok((input, (mapper_lo, flags[0] == 1, flags[1] == 1, flags[2] == 1, flags[3] == 1)))
}

fn parse_header(input: &[u8]) -> NomResult<&[u8], NesFileHeader> {
    let (input, (_, prg_size, chr_size)) = NomSeq::tuple((NomBytes::tag("NES\x1A"), NomNum::le_u8, NomNum::le_u8))(input)?;
    let (input, (mapper_lo, f, t, b, m)) = parse_flag6(input)?;
    Ok((input, NesFileHeader {
        prg_size, chr_size,
        mapper: mapper_lo as u16,
        is_four_screen: f,
        has_trainer: t,
        has_persistent_memory: b,
        mirroring: if m { Mirroring::Vertical } else { Mirroring::Horizontal },
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
        let mut data = BufReader::new("NES\x1A\x12\x34\x5C".as_bytes());
        let result = parse(&mut data).unwrap();
        assert_eq!(result.header.prg_size, 0x12);
        assert_eq!(result.header.chr_size, 0x34);
        assert_eq!(result.header.mapper, 0x5);
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
