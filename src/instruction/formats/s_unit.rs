use crate::instruction::{formats::FormatSymbol, parser::ParsingInstruction};

/// 1 or 2 sources instruction format.
pub const ONE_OR_TWO_SOURCES_FORMAT: [ParsingInstruction; 9] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 4,
        value: 0b1000,
    },
    ParsingInstruction::Bitfield {
        size: 6,
        symbol: FormatSymbol::Opfield,
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

/// ADDK instruction format.
pub const ADDK_FORMAT: [ParsingInstruction; 6] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 5,
        value: 0b10100,
    },
    ParsingInstruction::Bitfield {
        size: 16,
        symbol: FormatSymbol::Constant(16),
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::ConditionalOperation,
];

/// ADDKPC instruction format.
pub const ADDKPC_FORMAT: [ParsingInstruction; 7] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 11,
        value: 0b00001011000,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::Bitfield {
        size: 7,
        symbol: FormatSymbol::Source1,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::ConditionalOperation,
];

/// Extended .S unit 1 or 2 sources instruction format.
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
        value: 0b11,
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

/// Branch using a displacement instruction format.
pub const BRANCH_DISPLACEMENT_FORMAT: [ParsingInstruction; 5] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 5,
        value: 0b00100,
    },
    ParsingInstruction::Bitfield {
        size: 21,
        symbol: FormatSymbol::Constant(21),
    },
    ParsingInstruction::ConditionalOperation,
];

/// Branch using a register instruction format.
pub const BRANCH_REGISTER_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 10,
        value: 0b0011011000,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Crosspath,
    },
    ParsingInstruction::Match { size: 5, value: 0 },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::Match { size: 5, value: 0 },
    ParsingInstruction::ConditionalOperation,
];

/// Branch using a pointer instruction format.
pub const BRANCH_POINTER_FORMAT: [ParsingInstruction; 5] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Match {
        size: 17,
        value: 0b00000000001110001,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Match { size: 7, value: 0 },
    ParsingInstruction::ConditionalOperation,
];

/// BDEC/BPOS instruction format.
pub const BDEC_BPOS_POINTER_FORMAT: [ParsingInstruction; 7] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 10,
        value: 0b0000001000,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::IsBDec,
    },
    ParsingInstruction::Bitfield {
        size: 10,
        symbol: FormatSymbol::Source,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::ConditionalOperation,
];

/// Branch using a displacement with NOP instruction format.
pub const BRANCH_DISPLACEMENT_NOP_FORMAT: [ParsingInstruction; 6] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 11,
        value: 0b00001001000,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source1,
    },
    ParsingInstruction::Bitfield {
        size: 12,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::ConditionalOperation,
];

/// Branch using a register with NOP instruction format.
pub const BRANCH_REGISTER_NOP_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Match {
        size: 11,
        value: 0b00110110001,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Crosspath,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source1,
    },
    ParsingInstruction::Match { size: 2, value: 0 },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::Match {
        size: 5,
        value: 0b00001,
    },
    ParsingInstruction::ConditionalOperation,
];

/// Move constant instruction format
pub const MOVE_CONSTANT_FORMAT: [ParsingInstruction; 7] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 4,
        value: 0b1010,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::MoveHigh,
    },
    ParsingInstruction::Bitfield {
        size: 16,
        symbol: FormatSymbol::Constant(16),
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::ConditionalOperation,
];

/*
    16-bit instruction formats
*/

pub const SBS7_FORMAT: [ParsingInstruction; 4] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 5,
        value: 0b00101,
    },
    ParsingInstruction::Bitfield {
        size: 7,
        symbol: FormatSymbol::SignedConstant(7),
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::N3,
    },
];

pub const SBU8_FORMAT: [ParsingInstruction; 4] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 5,
        value: 0b00101,
    },
    ParsingInstruction::Bitfield {
        size: 8,
        symbol: FormatSymbol::UnsignedConstant(8),
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
];

pub const SCS10_FORMAT: [ParsingInstruction; 3] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 5,
        value: 0b01101,
    },
    ParsingInstruction::Bitfield {
        size: 10,
        symbol: FormatSymbol::SignedConstant(10),
    },
];

pub const SBS7C_FORMAT: [ParsingInstruction; 6] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 3,
        value: 0b101,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Zero,
    },
    ParsingInstruction::Match { size: 1, value: 1 },
    ParsingInstruction::Bitfield {
        size: 7,
        symbol: FormatSymbol::SignedConstant(7),
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::N3,
    },
];

pub const SBU8C_FORMAT: [ParsingInstruction; 6] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 3,
        value: 0b101,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Zero,
    },
    ParsingInstruction::Match { size: 1, value: 1 },
    ParsingInstruction::Bitfield {
        size: 8,
        symbol: FormatSymbol::UnsignedConstant(8),
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
];

pub const S3_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 3,
        value: 0b101,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Destination,
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
        symbol: FormatSymbol::Source1,
    },
];

pub const S3I_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 3,
        value: 0b101,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::Match { size: 1, value: 1 },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Crosspath,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Constant(3),
    },
];

pub const SMVK8_FORMAT: [ParsingInstruction; 7] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 4,
        value: 0b1001,
    },
    ParsingInstruction::BitfieldChunk {
        size: 2,
        symbol: FormatSymbol::UnsignedConstant(8),
        index: 5,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::BitfieldChunk {
        size: 1,
        symbol: FormatSymbol::UnsignedConstant(8),
        index: 7,
    },
    ParsingInstruction::BitfieldChunk {
        size: 2,
        symbol: FormatSymbol::UnsignedConstant(8),
        index: 3,
    },
    ParsingInstruction::BitfieldChunk {
        size: 3,
        symbol: FormatSymbol::UnsignedConstant(8),
        index: 0,
    },
];

pub const SX1_FORMAT: [ParsingInstruction; 5] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 6,
        value: 0b110111,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source2,
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

pub const SX1B_FORMAT: [ParsingInstruction; 5] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 6,
        value: 0b110111,
    },
    ParsingInstruction::Bitfield {
        size: 4,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::Match { size: 2, value: 0 },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::N3,
    },
];
