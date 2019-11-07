pub(super) mod parser;

use num_derive::FromPrimitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
pub enum Mirroring {
    Horizontal = 0,
    Vertical = 1,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
pub enum Timing {
    NTSC = 0x0,
    PAL = 0x1,
    MultipleRegion = 0x2,
    Dendy = 0x3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive)]
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
#[derive(Debug, Clone, Eq, PartialEq, FromPrimitive)]
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
    pub prg_rom_size: u32,
    pub chr_rom_size: u32,
    pub prg_ram_size: u32,
    pub prg_nv_ram_size: u32,
    pub chr_ram_size: u32,
    pub chr_nv_ram_size: u32,
    pub miscellaneous_rom_count: u8,
    pub mapper: u16,
    pub sub_mapper: u8,
    pub is_four_screen: bool,
    pub has_trainer: bool,
    pub has_persistent_memory: bool,
    pub mirroring: Mirroring,
    pub has_bus_conflicts: bool,
    pub timing: Timing,
    pub is_nes2: bool,
    pub console_type: ConsoleType,
    pub default_expansion_device: ExpansionDevice,
}
