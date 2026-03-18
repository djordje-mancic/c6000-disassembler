use crate::instruction::{formats::FormatSymbol, parser::ParsingInstruction};

/// 1 or 2 sources instruction format.
pub const ONE_OR_TWO_SOURCES_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 5,
        value: 0b10000,
    },
    ParsingInstruction::Bitfield {
        size: 6,
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Source1,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::ConditionalOperation,
];

/// Extended 1 or 2 sources instruction format.
pub const EXTENDED_ONE_OR_TWO_SOURCES_FORMAT: [ParsingInstruction; 10] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 4,
        value: 0b1100,
    },
    ParsingInstruction::Bitfield {
        size: 4,
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Crosspath,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Source1,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::ConditionalOperation,
];

/// ADDAB/ADDAH/ADDAW long-immediate operations instruction format
pub const ADDA_LONG_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::DUnitSide,
    },
    ParsingInstruction::Bitfield {
        size: 15,
        symbol: FormatSymbol::RegisterOffset,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::Match {
        size: 4,
        value: 0b0001,
    },
];

/// Linked word operations instruction format
pub const LINKED_WORD_FORMAT: [ParsingInstruction; 7] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Match {
        size: 6,
        value: 0b100001,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Match {
        size: 8,
        value: 0b00000001,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::BaseRegister,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::ConditionalOperation,
];

/// Load/Store basic operations instruction format.
pub const LOAD_STORE_BASIC_FORMAT: [ParsingInstruction; 11] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b01,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::DUnitSide,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::LoadStoreR,
    },
    ParsingInstruction::Bitfield {
        size: 4,
        symbol: FormatSymbol::AddressingMode,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::RegisterOffset,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::BaseRegister,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::ConditionalOperation,
];

/// Load/Store long-immediate operations instruction format.
pub const LOAD_STORE_LONG_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::DUnitSide,
    },
    ParsingInstruction::Bitfield {
        size: 15,
        symbol: FormatSymbol::RegisterOffset,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::ConditionalOperation,
];

/// Load/Store non-aligned doubleword instruction format.
pub const LOAD_STORE_NDW_FORMAT: [ParsingInstruction; 12] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b01,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::DUnitSide,
    },
    ParsingInstruction::BitMatch {
        symbol: FormatSymbol::LoadStoreR,
        value: true,
    },
    ParsingInstruction::Bitfield {
        size: 4,
        symbol: FormatSymbol::AddressingMode,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::RegisterOffset,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::BaseRegister,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::ScalingMode,
    },
    ParsingInstruction::Bitfield {
        size: 4,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::ConditionalOperation,
];

/*
    16-bit instruction formats
*/

pub const DOFF4_FORMAT: [ParsingInstruction; 10] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Pointer,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::DataSize,
    },
    ParsingInstruction::Match { size: 1, value: 0 },
    ParsingInstruction::BitfieldChunk {
        size: 1,
        symbol: FormatSymbol::UnsignedConstant(4),
        index: 3,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::BitfieldChunk {
        size: 3,
        symbol: FormatSymbol::UnsignedConstant(4),
        index: 0,
    },
];

pub const DOFF4DW_FORMAT: [ParsingInstruction; 11] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::NonAligned,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Pointer,
    },
    ParsingInstruction::BitMatch {
        symbol: FormatSymbol::DataSize,
        value: false,
    },
    ParsingInstruction::Match { size: 1, value: 0 },
    ParsingInstruction::BitfieldChunk {
        size: 1,
        symbol: FormatSymbol::UnsignedConstant(4),
        index: 3,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::BitfieldChunk {
        size: 3,
        symbol: FormatSymbol::UnsignedConstant(4),
        index: 0,
    },
];

pub const DIND_FORMAT: [ParsingInstruction; 9] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Pointer,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::DataSize,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b01,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source1,
    },
];

pub const DINDDW_FORMAT: [ParsingInstruction; 10] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::NonAligned,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Pointer,
    },
    ParsingInstruction::BitMatch {
        symbol: FormatSymbol::DataSize,
        value: false,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b01,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source1,
    },
];

pub const DINC_FORMAT: [ParsingInstruction; 10] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Pointer,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::DataSize,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::Bitfield {
        size: 1,
        symbol: FormatSymbol::UnsignedConstant(2),
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b00,
    },
];

pub const DINCDW_FORMAT: [ParsingInstruction; 11] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::NonAligned,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Pointer,
    },
    ParsingInstruction::BitMatch {
        symbol: FormatSymbol::DataSize,
        value: false,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::Bitfield {
        size: 1,
        symbol: FormatSymbol::UnsignedConstant(2),
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b00,
    },
];

pub const DDEC_FORMAT: [ParsingInstruction; 10] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Pointer,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::DataSize,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::Bitfield {
        size: 1,
        symbol: FormatSymbol::UnsignedConstant(2),
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b01,
    },
];

pub const DDECDW_FORMAT: [ParsingInstruction; 11] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::NonAligned,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Pointer,
    },
    ParsingInstruction::BitMatch {
        symbol: FormatSymbol::DataSize,
        value: false,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::Bitfield {
        size: 1,
        symbol: FormatSymbol::UnsignedConstant(2),
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b01,
    },
];

pub const DSTK_FORMAT: [ParsingInstruction; 9] = [
    ParsingInstruction::BitMatch {
        symbol: FormatSymbol::Side,
        value: true,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::BitfieldChunk {
        size: 3,
        symbol: FormatSymbol::UnsignedConstant(5),
        index: 2,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::BitfieldChunk {
        size: 2,
        symbol: FormatSymbol::UnsignedConstant(5),
        index: 0,
    },
    ParsingInstruction::Match { size: 1, value: 1 },
];

pub const DX2OP_FORMAT: [ParsingInstruction; 7] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 6,
        value: 0b011011,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::Match { size: 1, value: 0 },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Crosspath,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source1OrDestination,
    },
];

pub const DX5_FORMAT: [ParsingInstruction; 6] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 6,
        value: 0b011011,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::Match { size: 1, value: 1 },
    ParsingInstruction::BitfieldChunk {
        size: 2,
        symbol: FormatSymbol::UnsignedConstant(5),
        index: 3,
    },
    ParsingInstruction::BitfieldChunk {
        size: 3,
        symbol: FormatSymbol::UnsignedConstant(5),
        index: 0,
    },
];

pub const DX5P_FORMAT: [ParsingInstruction; 6] = [
    ParsingInstruction::BitMatch {
        symbol: FormatSymbol::Side,
        value: true,
    },
    ParsingInstruction::Match {
        size: 6,
        value: 0b111011,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::BitfieldChunk {
        size: 2,
        symbol: FormatSymbol::UnsignedConstant(5),
        index: 3,
    },
    ParsingInstruction::Match {
        size: 3,
        value: 0b011,
    },
    ParsingInstruction::BitfieldChunk {
        size: 3,
        symbol: FormatSymbol::UnsignedConstant(5),
        index: 0,
    },
];

pub const DX1_FORMAT: [ParsingInstruction; 5] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 6,
        value: 0b111011,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source2OrDestination,
    },
    ParsingInstruction::Match {
        size: 3,
        value: 0b110,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Opfield,
    },
];

pub const DPP_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::BitMatch {
        symbol: FormatSymbol::Side,
        value: true,
    },
    ParsingInstruction::Match {
        size: 6,
        value: 0b111011,
    },
    ParsingInstruction::Bitfield {
        size: 4,
        symbol: FormatSymbol::SourceOrDestination,
    },
    ParsingInstruction::Match { size: 1, value: 0 },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side2,
    },
    ParsingInstruction::Bitfield {
        size: 1,
        symbol: FormatSymbol::UnsignedConstant(2),
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsLoad,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsDoubleWord,
    },
];
