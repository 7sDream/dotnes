//! Struct and Enums to represent information stored in NES file's header segment

pub(super) mod parser;

use num_derive::FromPrimitive;

pub use parser::ParseHeaderError;

/// Name Table mirroring type
#[allow(missing_docs)] // because the variant name is clear enough
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Mirroring {
    HorizontalOrMapperControlled = 0,
    Vertical = 1,
}

/// CPU/PPU Timing
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromPrimitive)]
pub enum Timing {
    /// alias of RP2C02, used in North America, Japan, South Korea, Taiwan
    NTSC = 0x0,
    /// alias of RP2C07, used in Western Europe, Australia
    PAL = 0x1,
    /// Either if this game was released with identical ROM content in both NTSC and PAL countries
    /// or the game detects the console's timing and adjusts itself
    MultipleRegion = 0x2,
    /// alias of UMC 6527P, used in eastern Europe, Russia, Mainland China, India, Africa
    Dendy = 0x3,
}

/// Vs. System PPU type
#[allow(missing_docs)] // because the variant name is clear enough
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromPrimitive)]
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

/// Vs. System hardware type
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromPrimitive)]
pub enum VsHardwareType {
    /// Vs. Unisystem (normal)
    UniSystemNormal = 0x0,
    /// Vs. Unisystem (RBI Baseball protection)
    UniSystemRBIBaseballProtection = 0x1,
    /// Vs. Unisystem (TKO Boxing protection)
    UniSystemTKOBoxingProtection = 0x2,
    /// Vs. Unisystem (Super Xevious protection)
    UniSystemSuperXeviousProtection = 0x3,
    /// Vs. Unisystem (Vs. Ice Climber Japan protection)
    UniSystemVsIceClimberJapanProtection = 0x4,
    /// Vs. Dual System (normal)
    DualSystemNormal = 0x5,
    /// Vs. Dual System (Raid on Bungeling Bay protection)
    DualSystemRaidOnBungelingBayProtection = 0x6,
    /// Reserved
    Reserved = 0xFF,
}

/// Vs. System hardware information
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct VsInfo {
    /// PPU hardware type
    pub ppu_type: VsPPUType,
    /// If a game uses the DIP switches to select different PPU models, this field represents
    /// the correct PPU model when those DIP switches are all set to zero.
    pub hardware_type: VsHardwareType,
}

impl Default for VsInfo {
    #[must_use]
    fn default() -> Self {
        Self { ppu_type: VsPPUType::RP2C03B, hardware_type: VsHardwareType::UniSystemNormal }
    }
}

/// Console types which is other normal console type with some extends
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromPrimitive)]
pub enum ExtendedConsoleType {
    /// Regular NES/Famicom/Dendy
    Regular = 0x0,
    /// Vs. System
    Vs = 0x1,
    /// PlayChoice-10
    PC10 = 0x2,
    /// Regular Famiclone, but with CPU that supports Decimal Mode
    RegularWithDecimal = 0x3,
    /// V.R. Technology VT01 with monochrome palette
    VT01WithMonochrome = 0x4,
    /// V.R. Technology VT01 with red/cyan STN palette
    VT01WithRedCyanSTN = 0x5,
    /// V.R. Technology VT02
    VT02 = 0x6,
    /// V.R. Technology VT03
    VT03 = 0x7,
    /// V.R. Technology VT09
    VT09 = 0x8,
    /// V.R. Technology VT32
    VT32 = 0x9,
    /// V.R. Technology VT369
    VT369 = 0xA,
    /// Reserved
    Reserved = 0xFF,
}

/// Normal console types
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ConsoleType {
    /// Nintendo Entertainment System
    NES,
    /// Vs. System, has hardware info
    Vs(VsInfo),
    /// PlayChoice-10,
    PC10,
    /// Extend console types
    Extend(ExtendedConsoleType),
}

/// Devices may required by ROM when playing
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, FromPrimitive)]
pub enum ExpansionDevice {
    /// No special needs
    Unspecified = 0x00,
    /// Standard NES/Famicom controllers
    NES = 0x01,
    /// NES Four Score/Satellite with two additional standard controllers
    NESFourScore = 0x02,
    /// Famicom Four Players Adapter with two additional standard controllers
    FamicomFourPlayersAdapterWithTwoAdditionalStandardControllers = 0x03,
    /// Vs. System
    VsSystem = 0x04,
    /// Vs. System with reversed inputs
    VsSystemWithReversedInputs = 0x05,
    /// Vs. Pinball (Japan)
    VsPinballJapan = 0x06,
    /// Vs. Zapper
    VsZapper = 0x07,
    /// Zapper ($4017)
    Zapper = 0x08,
    /// Two Zappers
    TwoZappers = 0x09,
    /// Bandai Hyper Shot
    BandaiHyperShot = 0x0A,
    /// Power Pad Side A
    PowerPadSideA = 0x0B,
    /// Power Pad Side B
    PowerPadSideB = 0x0C,
    /// Family Trainer Side A
    FamilyTrainerSideA = 0x0D,
    /// Family Trainer Side B
    FamilyTrainerSideB = 0x0E,
    /// Arkanoid Vaus Controller (NES)
    ArkanoidVausControllerNES = 0x0F,
    /// Arkanoid Vaus Controller (Famicom)
    ArkanoidVausControllerFamicom = 0x10,
    /// Two Vaus Controllers plus Famicom Data Recorder
    TwoVausControllersPlusFamicomDataRecorder = 0x11,
    /// Konami Hyper Shot
    KonamiHyperShot = 0x12,
    /// Coconuts Pachinko Controller
    CoconutsPachinkoController = 0x13,
    /// Exciting Boxing Punching Bag
    ExcitingBoxingPunchingBag = 0x14,
    /// Jissen Mahjong Controller
    JissenMahjongController = 0x15,
    /// Party Tap
    PartyTap = 0x16,
    /// Oeka Kids Tablet
    OekaKidsTablet = 0x17,
    /// Sunsoft Barcode Battler
    SunsoftBarcodeBattler = 0x18,
    /// Miracle Piano Keyboard
    MiraclePianoKeyboard = 0x19,
    /// Pokkun Moguraa
    PokkunMoguraa = 0x1A,
    /// Top Rider
    TopRider = 0x1B,
    /// Double-Fisted
    DoubleFisted = 0x1C,
    /// Famicom 3D System
    Famicom3DSystem = 0x1D,
    /// Doremikko Keyboard
    DoremikkoKeyboard = 0x1E,
    /// R.O.B. Gyro Set
    ROBGyroSet = 0x1F,
    /// Famicom Data Recorder (don't emulate keyboard)
    FamicomDataRecorderDontEmulatekeyboard = 0x20,
    /// ASCII Turbo File
    ASCIITurboFile = 0x21,
    /// IGS Storage Battle Box
    IGSStorageBattleBox = 0x22,
    /// Family BASIC Keyboard plus Famicom Data Recorder
    FamilyBASICKeyboardPlusFamicomDataRecorder = 0x23,
    /// Dongda PEC-586 Keyboard
    DongdaPEC586Keyboard = 0x24,
    /// Bit Corp. Bit-79 Keyboard
    BitCorpBit79Keyboard = 0x25,
    /// Subor Keyboard
    SuborKeyboard = 0x26,
    /// Subor Keyboard plus mouse (3x8-bit protocol)
    SuborKeyboardPlus3x8BitProtocolMouse = 0x27,
    /// Subor Keyboard plus mouse (24-bit protocol)
    SuborKeyboardPlus24BitProtocolMouse = 0x28,
    /// SNES Mouse ($4017.d0)
    SNESMouse = 0x29,
    /// Multicart
    Multicart = 0x2A,
    /// Two SNES controllers replacing the two standard NES controllers
    TwoSNESControllersReplacingTheTwoStandardNESControllers = 0x2B,
    /// RacerMate Bicycle
    RacerMateBicycle = 0x2C,
    /// U-Force
    UForce = 0x2D,
    /// R.O.B. Stack-Up
    ROBStackUp = 0x2E,
    /// Reserved
    Reserved = 0xFF,
}

/// NES file format header info
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Header {
    /// PRG ROM size in bytes
    pub prg_rom_size: u32,
    /// CHR ROM size in bytes
    pub chr_rom_size: u32,
    /// PRG-RAM size in bytes
    pub prg_ram_size: u32,
    /// PRG-NVRAM size in bytes
    pub prg_nvram_size: u32,
    /// CHR-ROM size in bytes
    pub chr_ram_size: u32,
    /// CHR-NVRAM size in bytes
    pub chr_nvram_size: u32,
    /// Miscellaneous ROMs count in the end of file
    pub miscellaneous_rom_count: u8,
    /// NES Mapper index
    pub mapper: u16,
    /// Mapper sub index
    pub sub_mapper: u8,
    /// if name table mirroring use 4 screen mode
    pub is_four_screen: bool,
    /// if has trainer data
    pub has_trainer: bool,
    /// if has persistent memory
    pub has_persistent_memory: bool,
    /// name table mirroring mode
    pub mirroring: Mirroring,
    /// if has bus conflicts
    pub has_bus_conflicts: bool,
    /// CPU/PPU timing
    pub timing: Timing,
    /// if is NES 2.0 format
    pub is_nes2: bool,
    /// Console type the game runs on
    pub console_type: ConsoleType,
    /// Required devices when playing this game
    pub default_expansion_device: ExpansionDevice,
}
