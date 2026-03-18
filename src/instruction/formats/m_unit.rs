use crate::instruction::{formats::FormatSymbol, parser::ParsingInstruction};

/// Extended .M unit unary instruction format.
pub const EXTENDED_UNARY_FORMAT: [ParsingInstruction; 8] = [
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Parallel,
    },
    ParsingInstruction::Bit {
        symbol: FormatSymbol::Side,
    },
    ParsingInstruction::Match {
        size: 10,
        value: 0b0000111100,
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
