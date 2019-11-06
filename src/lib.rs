#![deny(warnings)]

use std::{
    convert::{From, TryFrom},
};

use nom::{
    bits as NomConvert, bits::streaming as NomBits, bytes::streaming as NomBytes,
    error::ErrorKind as NomErrorKind, number::complete as NomNum,
    sequence as NomSeq, Err as NomErr, IResult as NomResult, Slice as NomSlice,
};

use num_enum::TryFromPrimitive;

const KB: u32 = 1 << 10;
const TRAINER_SIZE: u32 = 512;

#[derive(Debug)]
pub enum ParseError {
    DataInvalid(String),
}

impl<T> From<NomErr<(T, NomErrorKind)>> for ParseError {
    fn from(err: NomErr<(T, NomErrorKind)>) -> Self {
        ParseError::DataInvalid(match err {
            NomErr::Incomplete(_) => "There was not enough data".to_string(),
            NomErr::Failure((_, e)) | NomErr::Error((_, e)) => e.description().to_string(),
        })
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum Mirroring {
    Horizontal = 0,
    Vertical = 1,
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
    Reserved = 0xFF,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum VsHardwareType {
    UniSystemNormal = 0x0,
    UniSystemRBIBaseballProtection = 0x1,
    UniSystemTKOBoxingProtection = 0x2,
    UniSystemSuperXeviousProtection = 0x3,
    UniSystemVsIceClimberJapanProtection = 0x4,
    DualSystemNormal = 0x5,
    DualSystemRaidOnBungelingBayProtection = 0x6,
    Reserved = 0xFF,
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
    Reversed = 0xFF,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ConsoleType {
    Nes,
    Vs(VsInfo),
    Pc10,
    Extend(ExtendedConsoleType),
}

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, TryFromPrimitive)]
pub enum ExpansionDevice {
    Unspecified = 0x00,
    NES = 0x01,
    NESFourScore = 0x02,
    FamicomFourPlayersAdapterWithTwoAdditionalStandardControllers = 0x03,
    VsSystem = 0x04,
    VsSystemWithReversedInputs = 0x05,
    VsPinballJapan = 0x06,
    VsZapper = 0x07,
    Zapper4017 = 0x08,
    TwoZappers = 0x09,
    BandaiHyperShot = 0x0A,
    PowerPadSideA = 0x0B,
    PowerPadSideB = 0x0C,
    FamilyTrainerSideA = 0x0D,
    FamilyTrainerSideB = 0x0E,
    ArkanoidVausControllerNES = 0x0F,
    ArkanoidVausControllerFamicom = 0x10,
    TwoVausControllersPlusFamicomDataRecorder = 0x11,
    KonamiHyperShot = 0x12,
    CoconutsPachinkoController = 0x13,
    ExcitingBoxingPunchingBag = 0x14,
    JissenMahjongController = 0x15,
    PartyTap = 0x16,
    OekaKidsTablet = 0x17,
    SunsoftBarcodeBattler = 0x18,
    MiraclePianoKeyboard = 0x19,
    PokkunMoguraa = 0x1A,
    TopRider = 0x1B,
    DoubleFisted = 0x1C,
    Famicom3DSystem = 0x1D,
    DoremikkoKeyboard = 0x1E,
    ROBGyroSet = 0x1F,
    FamicomDataRecorderDontEmulatekeyboard = 0x20,
    ASCIITurboFile = 0x21,
    IGSStorageBattleBox = 0x22,
    FamilyBASICKeyboardPlusFamicomDataRecorder = 0x23,
    DongdaPEC586Keyboard = 0x24,
    BitCorpBit79Keyboard = 0x25,
    SuborKeyboard = 0x26,
    SuborKeyboardPlus3x8BitProtocolMouse = 0x27,
    SuborKeyboardPlus24BitProtocolMouse = 0x28,
    SNESMouse = 0x29,
    Multicart = 0x2A,
    TwoSNESControllersReplacingTheTwoStandardNESControllers = 0x2B,
    RacerMateBicycle = 0x2C,
    UForce = 0x2D,
    ROBStackUp = 0x2E,
    Reversed = 0xFF,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NesFileHeader {
    prg_rom_size: u32,
    chr_rom_size: u32,
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
    has_bus_conflicts: bool,
    timing: Timing,
    is_nes2: bool,
    console_type: ConsoleType,
    default_expansion_device: ExpansionDevice,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NesFile<'a> {
    pub header: NesFileHeader,
    pub trainer: Option<&'a [u8]>,
    pub prg_rom: &'a [u8],
    pub chr_rom: &'a [u8],
    pub miscellaneous: &'a [u8],
}

fn bits_tuple<I, O, L>(l: L) -> impl Fn(I) -> NomResult<I, O>
where
    I: NomSlice<std::ops::RangeFrom<usize>> + Clone,
    L: NomSeq::Tuple<(I, usize), O, ((I, usize), NomErrorKind)> {
    NomConvert::bits(NomSeq::tuple::<_, _, ((I, usize), NomErrorKind), _>(l))
}

fn parse_flag6_common(input: &[u8]) -> NomResult<&[u8], (u8, u8, u8, u8, u8)> {
    bits_tuple((
        NomBits::take(4u8),
        NomBits::take(1u8),
        NomBits::take(1u8),
        NomBits::take(1u8),
        NomBits::take(1u8),
    ))(input)
}

fn parse_flag7_common(input: &[u8]) -> NomResult<&[u8], (u8, u8, u8)> {
    bits_tuple((NomBits::take(4u8), NomBits::take(2u8), NomBits::take(2u8)))(input)
}

fn parse_flag8_nes2(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((NomBits::take(4u8), NomBits::take(4u8)))(input)
}

fn parse_flag8_nes1(input: &[u8]) -> NomResult<&[u8], u8> {
    NomNum::be_u8(input)
}

fn parse_flag9_nes2(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((NomBits::take(4u8), NomBits::take(4u8)))(input)
}

fn parse_flag9_nes1(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((NomBits::take(7u8), NomBits::take(1u8)))(input)
}

fn parse_flag10_nes2(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((NomBits::take(4u8), NomBits::take(4u8)))(input)
}

fn parse_flag10_nes1(input: &[u8]) -> NomResult<&[u8], (u8, u8, u8)> {
    let (input, (_, bus_conflicts, no_prg_ram, _, tv_system)) = bits_tuple((
        NomBits::take::<_, u8, _, _>(2u8),
        NomBits::take(1u8),
        NomBits::take(1u8),
        NomBits::take::<_, u8, _, _>(2u8),
        NomBits::take(2u8)
    ))(input)?;
    Ok((input, (bus_conflicts, no_prg_ram, tv_system)))
}

fn parse_flag11_nes2(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((NomBits::take(4u8), NomBits::take(4u8)))(input)
}

fn parse_flag12_nes2(input: &[u8]) -> NomResult<&[u8], u8> {
    let (input, (_unused, timing)): (_, (u8, _)) =
        bits_tuple((NomBits::take(6u8), NomBits::take(2u8)))(input)?;
    Ok((input, timing))
}

fn parse_flag13_nes2(input: &[u8]) -> NomResult<&[u8], (u8, u8)> {
    bits_tuple((NomBits::take(4u8), NomBits::take(4u8)))(input)
}

fn parse_flag14_nes2(input: &[u8]) -> NomResult<&[u8], u8> {
    let (input, (_unused, timing)): (_, (u8, _)) =
        bits_tuple((NomBits::take(6u8), NomBits::take(2u8)))(input)?;
    Ok((input, timing))
}

fn parse_flag15_nes2(input: &[u8]) -> NomResult<&[u8], u8> {
    let (input, (_unused, device)): (_, (u8, _)) =
        bits_tuple((NomBits::take(2u8), NomBits::take(6u8)))(input)?;
    Ok((input, device))
}

fn nes1_ignore_flag_11_to_15(input: &[u8]) -> NomResult<&[u8], &[u8]> {
    NomBytes::take(5u8)(input)
}

fn parse_header(input: &[u8]) -> NomResult<&[u8], NesFileHeader> {
    let (input, (_, prg_rom_size_lo, chr_rom_size_lo)) =
        NomSeq::tuple((NomBytes::tag("NES\x1A"), NomNum::le_u8, NomNum::le_u8))(input)?;
    let mut prg_rom_size = prg_rom_size_lo as u32;
    let mut chr_rom_size = chr_rom_size_lo as u32;

    let (input, (mapper_lo, f, t, b, m)) = parse_flag6_common(input)?;

    let (input, (mut mapper_mid, nes2, console_type)) = parse_flag7_common(input)?;
    let is_nes2 = nes2 == 0b10;
    let mut console = match console_type {
        0 => ConsoleType::Nes,
        1 => ConsoleType::Vs(VsInfo::default()),
        2 => ConsoleType::Pc10,
        3 => {
            if is_nes2 {
                ConsoleType::Extend(ExtendedConsoleType::Regular)
            } else {
                // FIXME: Can iNES 1.0 format's console_type bits be 0b11?
                ConsoleType::Vs(VsInfo::default())
            }
        }
        _ => unreachable!("console type must in 0 - 3"),
    };

    let mut sub_mapper = 0;
    let mut prg_ram_size: u32 = 0;
    let mut prg_nv_ram_size: u32 = 0;
    let mut chr_ram_size: u32 = 0;
    let mut chr_nv_ram_size: u32 = 0;
    let timing;
    let mut has_bus_conflicts = false;
    let mut miscellaneous_rom_count = 0;
    let mut default_expansion_device = ExpansionDevice::Unspecified;

    if is_nes2 {
        let (input, (sub_mapper_actual, mapper_hi)) = parse_flag8_nes2(input)?;
        sub_mapper = sub_mapper_actual;
        mapper_mid |= mapper_hi << 4;

        let (input, (prg_rom_size_hi, chr_rom_size_hi)) = parse_flag9_nes2(input)?;
        prg_rom_size |= (prg_rom_size_hi as u32) << 8;
        chr_rom_size |= (chr_rom_size_hi as u32) << 8;

        let (input, (prg_ram_shift, prg_nv_ram_shift)) = parse_flag10_nes2(input)?;
        if prg_ram_shift != 0 {
            prg_ram_size = 64u32 << prg_ram_shift as u32;
        }
        if prg_nv_ram_shift != 0 {
            prg_nv_ram_size = 64u32 << prg_nv_ram_shift as u32;
        }

        let (input, (chr_ram_shift, chr_nv_ram_shift)) = parse_flag11_nes2(input)?;
        if chr_ram_shift != 0 {
            chr_ram_size = 64u32 << chr_ram_shift as u32;
        }
        if chr_nv_ram_shift != 0 {
            chr_nv_ram_size = 64u32 << chr_nv_ram_shift as u32;
        }

        let (input, timing_actual) = parse_flag12_nes2(input)?;
        timing = Timing::try_from(timing_actual).unwrap();

        let (input, (a, b)) = parse_flag13_nes2(input)?;
        if let ConsoleType::Vs(ref mut info) = console {
            info.hardware_type = VsHardwareType::try_from(a).unwrap_or(VsHardwareType::Reserved);
            info.ppu_type = VsPPUType::try_from(b).unwrap_or(VsPPUType::Reserved);
        } else if let ConsoleType::Extend(ref mut extend) = console {
            *extend = ExtendedConsoleType::try_from(b).unwrap_or(ExtendedConsoleType::Reversed);
        }

        let (input, miscellaneous_rom_count_actual) = parse_flag14_nes2(input)?;
        miscellaneous_rom_count = miscellaneous_rom_count_actual;

        let (_input, device) = parse_flag15_nes2(input)?;
        default_expansion_device =
            ExpansionDevice::try_from(device).unwrap_or(ExpansionDevice::Reversed);
    } else {
        let (input, prg_ram_size_actual) = parse_flag8_nes1(input)?;
        // 8KB per unit, 0 as 8KB
        prg_ram_size = u32::max(1, prg_ram_size_actual as u32) * 8 * 1024;

        let (input, (_reserved, _timing_actual)) = parse_flag9_nes1(input)?;

        let (input, (bus_conflicts, no_prg_ram, timing_actual)) = parse_flag10_nes1(input)?;
        has_bus_conflicts = bus_conflicts != 0;
        if no_prg_ram != 0 {
            prg_ram_size = 0;
        }
        timing = match timing_actual {
            0 => Timing::NTSC,
            2 => Timing::PAL,
            1 | 3 => Timing::MultipleRegion,
            _ => unreachable!(),
        };

        let (_input, _) = nes1_ignore_flag_11_to_15(input)?;
    }

    prg_rom_size = if prg_rom_size >> 8 == 0xF {
        // size = 2^E *(MM*2+1)
        let mm = prg_rom_size & 0b00000011;
        let e = (prg_rom_size & 0xF) >> 2;
        (1 << e) * (mm * 2 + 1)
    } else {
        prg_rom_size * 16 * KB
    };

    chr_rom_size = if chr_rom_size >> 8 == 0xF {
        // size = 2^E *(MM*2+1)
        let mm = chr_rom_size & 0b00000011;
        let e = chr_rom_size >> 2;
        (1 << e) * (mm * 2 + 1)
    } else {
        chr_rom_size * 16 * KB
    };

    Ok((
        input,
        NesFileHeader {
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
            mirroring: Mirroring::try_from(m).unwrap(),
            has_bus_conflicts,
            timing,
            is_nes2,
            console_type: console,
            default_expansion_device,
        },
    ))
}

pub fn parse<'a, I: AsRef<[u8]>>(input: &I) -> Result<NesFile, ParseError> {
    let input = input.as_ref();
    let (input, header) = parse_header(input)?;

    let (input, trainer) = if header.has_trainer {
        let (next_input, trainer_body) = NomBytes::take(TRAINER_SIZE)(input)?;
        (next_input, Some(trainer_body))
    } else {
        (input, None)
    };

    let (input, prg_rom) = NomBytes::take(header.prg_rom_size)(input)?;

    let (input, chr_rom) = NomBytes::take(header.chr_rom_size)(input)?;

    let miscellaneous = input;

    Ok(NesFile { header,  trainer, prg_rom, chr_rom, miscellaneous})
}
