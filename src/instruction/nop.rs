use crate::instruction::{
    C6000Instruction, InstructionData,
    formats::{
        FormatSymbol,
        no_unit::{IDLE_NOP_FORMAT, UNOP_FORMAT},
    },
    parser::parse,
};
use std::io::{Error, ErrorKind, Result};

pub struct NOPInstruction {
    pub count: u8,
    instruction_data: InstructionData,
}

impl C6000Instruction for NOPInstruction {
    fn new(input: &super::InstructionInput) -> Result<Self> {
        let parsed_variables = parse(input.opcode, &IDLE_NOP_FORMAT)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Not a NOP/IDLE: {e}")))?;
        let p_bit = parsed_variables.try_get_bool(FormatSymbol::Parallel)?;
        let count = parsed_variables.try_get_u8(FormatSymbol::Opfield)?;
        Ok(NOPInstruction {
            count,
            instruction_data: InstructionData {
                opcode: input.opcode,
                p_bit,
                ..Default::default()
            },
        })
    }

    fn new_compact(input: &super::InstructionInput) -> Result<Self> {
        let parsed_variables = parse(input.opcode, &UNOP_FORMAT)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, format!("Not a NOP/IDLE: {e}")))?;
        let count = parsed_variables.try_get_u8(FormatSymbol::N3)?;
        Ok(NOPInstruction {
            count,
            instruction_data: InstructionData {
                opcode: input.opcode,
                compact: true,
                ..Default::default()
            },
        })
    }

    fn instruction(&self) -> String {
        if self.count == 0b1111 {
            String::from("IDLE")
        } else {
            String::from("NOP")
        }
    }

    fn operands(&self) -> String {
        if self.count > 0 && self.count != 0b1111 {
            format!("{}", self.count + 1)
        } else {
            String::new()
        }
    }

    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}
