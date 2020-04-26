#[macro_use]
mod byte_splitter_gen;
mod common;
mod v1;
mod v2;

use {
    super::{
        ConsoleType, ExpansionDevice, ExtendedConsoleType, Header, Mirroring, Timing,
        VsHardwareType, VsInfo, VsPPUType,
    },
    num_traits::FromPrimitive,
};

/// Parse head failed reason
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ParseHeaderError {
    MagicConstantNotMatch,
    TwoDifferTiming,
}

const NES_MAGIC_CONSTANT: &[u8; 4] = b"NES\x1A";
const NES_V2_IDENTIFIER: u8 = 0b10;
const KB: u32 = 1 << 10;

#[allow(clippy::similar_names)] // for `rom` and `ram` is similar
#[allow(clippy::too_many_lines)] // TODO: reduce code lines
pub fn parse_header(input: &[u8]) -> Result<Header, ParseHeaderError> {
    if !input.starts_with(NES_MAGIC_CONSTANT) {
        return Err(ParseHeaderError::MagicConstantNotMatch);
    }

    let prg_rom_size = u32::from(input[4]);
    let chr_rom_size = u32::from(input[5]);

    let [mapper_low, four_screen, trainer, battery, mirroring] = common::flag6(input[6]);
    let is_four_screen = four_screen == 1;
    let has_trainer = trainer == 1;
    let has_persistent_memory = battery == 1;
    let mirroring = Mirroring::from_u8(mirroring).unwrap();

    let [mapper_mid, nes2, console_type] = common::flag7(input[7]);
    let is_nes2 = nes2 == NES_V2_IDENTIFIER;
    let mapper = u16::from(mapper_mid << 4 | mapper_low);
    let mut console_type = match console_type {
        0 => ConsoleType::NES,
        1 => ConsoleType::Vs(VsInfo::default()),
        2 => ConsoleType::PC10,
        3 => {
            if is_nes2 {
                ConsoleType::Extend(ExtendedConsoleType::Regular)
            } else {
                // FIXME: Can NES 1.0 format's console_type bits be 0b11?
                ConsoleType::Vs(VsInfo::default())
            }
        }
        _ => unreachable!("console type only two bits, must in 0 - 3"),
    };

    if is_nes2 {
        let [sub_mapper, mapper_high] = v2::flag8(input[8]);
        let [prg_rom_size_hi, chr_rom_size_hi] = v2::flag9(input[9]);
        let [prg_ram_shift, prg_nvram_shift] = v2::flag10(input[10]);
        let [chr_ram_shift, chr_nvram_shift] = v2::flag11(input[11]);
        let [_, timing] = v2::flag12(input[12]);
        let [a, b] = v2::flag13(input[13]);
        let [_, miscellaneous_rom_count] = v2::flag14(input[14]);
        let [_, default_expansion_device] = v2::flag15(input[14]);

        let mapper = mapper | (u16::from(mapper_high) << 8);

        let prg_rom_size = if prg_rom_size_hi == 0xF {
            let mm = prg_rom_size & 0x3;
            let e = prg_rom_size >> 2;
            (1 << e) * (mm * 2 + 1)
        } else {
            (prg_rom_size | (u32::from(prg_rom_size_hi) << 8)) * 16 * KB
        };

        let chr_rom_size = if chr_rom_size_hi == 0xF {
            let mm = chr_rom_size & 0x3;
            let e = chr_rom_size >> 2;
            (1 << e) * (mm * 2 + 1)
        } else {
            (chr_rom_size | ((chr_rom_size as u32) << 8)) * 8 * KB
        };

        let prg_ram_size = if prg_ram_shift == 0 { 0 } else { 64_u32 << u32::from(prg_ram_shift) };

        let prg_nvram_size =
            if prg_nvram_shift == 0 { 0 } else { 64_u32 << u32::from(prg_nvram_shift) };

        let chr_ram_size = if chr_ram_shift == 0 { 0 } else { 64_u32 << u32::from(chr_ram_shift) };

        let chr_nvram_size =
            if chr_nvram_shift == 0 { 0 } else { 64_u32 << u32::from(chr_nvram_shift) };

        let timing = Timing::from_u8(timing).unwrap();

        if let ConsoleType::Vs(ref mut info) = console_type {
            info.hardware_type = VsHardwareType::from_u8(a).unwrap_or(VsHardwareType::Reserved);
            info.ppu_type = VsPPUType::from_u8(b).unwrap_or(VsPPUType::Reserved);
        } else if let ConsoleType::Extend(ref mut extend) = console_type {
            *extend = ExtendedConsoleType::from_u8(b).unwrap_or(ExtendedConsoleType::Reversed);
        }

        let default_expansion_device =
            ExpansionDevice::from_u8(default_expansion_device).unwrap_or(ExpansionDevice::Reversed);

        Ok(Header {
            prg_rom_size,
            chr_rom_size,
            prg_ram_size,
            prg_nvram_size,
            chr_ram_size,
            chr_nvram_size,
            miscellaneous_rom_count,
            mapper,
            sub_mapper,
            is_four_screen,
            has_trainer,
            has_persistent_memory,
            mirroring,
            has_bus_conflicts: false,
            timing,
            is_nes2,
            console_type,
            default_expansion_device,
        })
    } else {
        let [mut prg_ram_size] = v1::flag8(input[8]);
        let [_, timing1] = v1::flag9(input[9]);
        let [_, bus_conflicts, no_prg_ram, _, timing2] = v1::flag10(input[10]);

        // NES 1.0 don't use flag 11 - 15

        let prg_rom_size = prg_rom_size * 16 * KB;
        let chr_rom_size = chr_rom_size * 8 * KB;

        let has_bus_conflicts = bus_conflicts != 0;

        prg_ram_size = u8::max(prg_ram_size, 1);
        if no_prg_ram != 0 {
            prg_ram_size = 0;
        }
        let prg_ram_size = u32::from(prg_ram_size) * 8 * KB;

        if timing1 != 0 && timing2 != 0 && timing1 != timing2 {
            return Err(ParseHeaderError::TwoDifferTiming);
        }
        let timing = u8::max(timing1, timing2);
        let timing = match timing {
            0 => Timing::NTSC,
            2 => Timing::PAL,
            1 | 3 => Timing::MultipleRegion,
            _ => unreachable!(),
        };

        Ok(Header {
            prg_rom_size,
            chr_rom_size,
            prg_ram_size,
            prg_nvram_size: 0,
            chr_ram_size: 0,
            chr_nvram_size: 0,
            miscellaneous_rom_count: 0,
            mapper,
            sub_mapper: 0,
            is_four_screen,
            has_trainer,
            has_persistent_memory,
            mirroring,
            has_bus_conflicts,
            timing,
            is_nes2,
            console_type,
            default_expansion_device: ExpansionDevice::Unspecified,
        })
    }
}
