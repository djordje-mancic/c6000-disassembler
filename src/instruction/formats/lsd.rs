use crate::instruction::{formats::FormatSymbol, parser::ParsingInstruction};

pub const LSDMVTO_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Unit,
    },
    ParsingInstruction::Match { size: 2, value: 0 },
    ParsingInstruction::BitfieldChunk {
        size: 3,
        symbol: FormatSymbol::Source2,
        index: 0,
    },
    ParsingInstruction::BitfieldChunk {
        size: 2,
        symbol: FormatSymbol::Source2,
        index: 3,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Crosspath,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Destination,
    },
];

pub const LSDMVFR_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Unit,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b10,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source2,
    },
    ParsingInstruction::BitfieldChunk {
        size: 2,
        symbol: FormatSymbol::Destination,
        index: 3,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Crosspath,
    },
    ParsingInstruction::BitfieldChunk {
        size: 3,
        symbol: FormatSymbol::Destination,
        index: 0,
    },
];

pub const LSDX1C_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Unit,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Destination,
    },
    ParsingInstruction::Match {
        size: 3,
        value: 0b010,
    },
    ParsingInstruction::Bitfield {
        size: 1,
        symbol: FormatSymbol::UnsignedConstant(1),
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::CC,
    },
];

pub const LSDX1_FORMAT: [ParsingInstruction; 7] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bitfield {
        size: 2,
        symbol: FormatSymbol::Unit,
    },
    ParsingInstruction::Match {
        size: 2,
        value: 0b11,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::Source,
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
