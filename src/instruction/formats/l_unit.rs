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
        size: 3,
        value: 0b110,
    },
    ParsingInstruction::Bitfield {
        size: 7,
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

pub const UNARY_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 10,
        value: 0b0011010110,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Crosspath,
    },
    ParsingInstruction::Bitfield {
        size: 5,
        symbol: FormatSymbol::Opfield,
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

/*
    16-bit instruction formats
*/

pub const L3_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match { size: 3, value: 0 },
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

pub const L3I_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match { size: 3, value: 0 },
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
        symbol: FormatSymbol::Source1,
    },
];

pub const L2C_FORMAT: [ParsingInstruction; 9] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 3,
        value: 0b100,
    },
    ParsingInstruction::Bitfield {
        size: 1,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::BitfieldChunk {
        size: 2,
        symbol: FormatSymbol::Opfield,
        index: 0,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::Match { size: 1, value: 1 },
    ParsingInstruction::BitfieldChunk {
        size: 1,
        symbol: FormatSymbol::Opfield,
        index: 2,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Crosspath,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source1,
    },
];

pub const LX5_FORMAT: [ParsingInstruction; 6] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 6,
        value: 0b010011,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::Match { size: 1, value: 1 },
    ParsingInstruction::BitfieldChunk {
        size: 2,
        symbol: FormatSymbol::SignedConstant(5),
        index: 3,
    },
    ParsingInstruction::BitfieldChunk {
        size: 3,
        symbol: FormatSymbol::SignedConstant(5),
        index: 0,
    },
];
