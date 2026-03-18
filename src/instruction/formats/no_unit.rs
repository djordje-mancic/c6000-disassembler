use crate::instruction::{formats::FormatSymbol, parser::ParsingInstruction};

pub const IDLE_NOP_FORMAT: [ParsingInstruction; 4] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Match { size: 12, value: 0 },
    ParsingInstruction::Bitfield {
        size: 4,
        symbol: FormatSymbol::Opfield,
    },
    ParsingInstruction::Match { size: 15, value: 0 },
];

pub const UNOP_FORMAT: [ParsingInstruction; 2] = [
    ParsingInstruction::Match {
        size: 13,
        value: 0xC6E,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::N3,
    },
];

pub const FPHEAD_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::BitArray {
        size: 14,
        symbol: FormatSymbol::FPHeadPBits,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::FPHeadSaturate,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::FPHeadBranches,
    },
    ParsingInstruction::Bitfield {
        size: 3,
        symbol: FormatSymbol::FPHeadDataSizes,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::FPHeadRegisterSet,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::FPHeadLoadsProtected,
    },
    ParsingInstruction::BitArray {
        size: 7,
        symbol: FormatSymbol::FPHeadLayout,
    },
    ParsingInstruction::Match {
        size: 4,
        value: 0b1110,
    },
];
