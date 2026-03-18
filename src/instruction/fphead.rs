use std::io::{Error, ErrorKind, Result};

use crate::instruction::{
    C6000Instruction, DataSize, InstructionData,
    formats::{FormatSymbol, no_unit::FPHEAD_FORMAT},
    parser::parse,
};

#[derive(Clone)]
pub struct CompactInstructionHeader {
    instruction_data: InstructionData,
    /// Layout field
    /// Determines if the i-th word holds two compact (16-bit) instructions (true)
    /// or a regular, 32-bit instruction (false).
    pub layout: [bool; 7],
    /// PROT field.
    /// Determines if 4 ``NOP`` cycles are added after every LD instruction.
    pub loads_protected: bool,
    /// RS field.
    /// Determines if instructions use high register set for data source
    /// and destination (true) or low register set (false).
    pub register_set: bool,
    pub primary_data_size: DataSize,
    pub secondary_data_size: DataSize,
    /// BR field.
    /// Determines if compact instructions in the S unit are decoded
    /// as branches.
    pub decode_compact_branches: bool,
    /// SAT field.
    /// Determines if instructions are saturated.
    ///
    /// As a result, ``ADD``, ``SUB``, ``SHL``, ``MPY``, ``MPYH``, ``MPYLH`` and ``MPYHL``
    /// instructions are decoded as ``SADD``, ``SUBS``, ``SSHL``, ``SMPY``, ``SMPYH``, ``SMPYLH`` and
    /// ``SMPYHL`` respectively.
    pub saturate: bool,
    pub compact_p_bits: [bool; 14],
}

impl C6000Instruction for CompactInstructionHeader {
    fn new(input: &super::InstructionInput) -> Result<Self> {
        let parsed_variables = parse(input.opcode, &FPHEAD_FORMAT).map_err(|e| {
            Error::new(
                ErrorKind::InvalidInput,
                format!("Not a compact instruction header: {e}"),
            )
        })?;

        let layout = {
            let layout_vec = parsed_variables.try_get_bool_vec(FormatSymbol::FPHeadLayout)?;
            let Some(layout_ref) = layout_vec.first_chunk::<7>() else {
                return Err(Error::other("Layout doesn't have 7 elements"));
            };
            *layout_ref
        };
        let compact_p_bits = {
            let layout_vec = parsed_variables.try_get_bool_vec(FormatSymbol::FPHeadPBits)?;
            let Some(layout_ref) = layout_vec.first_chunk::<14>() else {
                return Err(Error::other("P-bits don't have 14 elements"));
            };
            *layout_ref
        };
        let loads_protected = parsed_variables.try_get_bool(FormatSymbol::FPHeadLoadsProtected)?;
        let register_set = parsed_variables.try_get_bool(FormatSymbol::FPHeadRegisterSet)?;
        let data_sizes = parsed_variables.try_get_u8(FormatSymbol::FPHeadDataSizes)?;
        let primary_data_size = {
            if data_sizes & 0b100 != 0 {
                DataSize::DoubleWord
            } else {
                DataSize::Word
            }
        };
        let secondary_data_size = {
            if primary_data_size == DataSize::DoubleWord {
                match data_sizes & 0b11 {
                    0 => DataSize::Word,
                    1 => DataSize::Byte,
                    2 => DataSize::NonAlignedWord,
                    3 => DataSize::HalfWord,
                    _ => DataSize::Word,
                }
            } else {
                match data_sizes & 0b11 {
                    0 => DataSize::ByteUnsigned,
                    1 => DataSize::Byte,
                    2 => DataSize::HalfWordUnsigned,
                    3 => DataSize::HalfWord,
                    _ => DataSize::ByteUnsigned,
                }
            }
        };
        let decode_compact_branches =
            parsed_variables.try_get_bool(FormatSymbol::FPHeadBranches)?;
        let saturate = parsed_variables.try_get_bool(FormatSymbol::FPHeadSaturate)?;
        Ok(Self {
            instruction_data: InstructionData {
                opcode: input.opcode,
                ..Default::default()
            },
            layout,
            compact_p_bits,
            loads_protected,
            register_set,
            primary_data_size,
            secondary_data_size,
            decode_compact_branches,
            saturate,
        })
    }

    fn instruction(&self) -> String {
        String::from(".fphead")
    }
    fn operands(&self) -> String {
        let mut layout_str = String::new();
        for i in (0..7).rev() {
            if self.layout[i] {
                layout_str += "1";
            } else {
                layout_str += "0";
            }
        }
        format!(
            "{}, {}, {}, {}, {}, {}, {layout_str}",
            if self.loads_protected { "p" } else { "n" },
            if self.register_set { "h" } else { "l" },
            self.primary_data_size.to_short_string(),
            self.secondary_data_size.to_short_string(),
            if self.decode_compact_branches {
                "br"
            } else {
                "nobr"
            },
            if self.saturate { "sat" } else { "nosat" }
        )
    }
    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }
    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}
