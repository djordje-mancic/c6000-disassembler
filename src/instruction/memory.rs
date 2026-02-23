use std::{
    fmt::Display,
    io::{Error, ErrorKind, Result},
};

use crate::instruction::{
    C64xInstruction, DataSize, InstructionData,
    parser::{ParsedVariable, ParsingInstruction, parse},
    register::Register,
};

#[derive(PartialEq, Eq)]
pub enum MemoryInstructionType {
    Load,
    Store,
}

pub enum AddressGeneratorMode {
    NegativeR(Register),
    PositiveR(Register),
    PredecrementR(Register),
    PreincrementR(Register),
    PostdecrementR(Register),
    PostincrementR(Register),
    Negative(u32),
    Positive(u32),
    Predecrement(u32),
    Preincrement(u32),
    Postdecrement(u32),
    Postincrement(u32),
}

impl AddressGeneratorMode {
    pub fn get_register(&self) -> Option<Register> {
        match self {
            AddressGeneratorMode::NegativeR(register)
            | AddressGeneratorMode::PositiveR(register)
            | AddressGeneratorMode::PredecrementR(register)
            | AddressGeneratorMode::PreincrementR(register)
            | AddressGeneratorMode::PostdecrementR(register)
            | AddressGeneratorMode::PostincrementR(register) => Some(*register),
            _ => None,
        }
    }

    pub fn get_constant(&self) -> Option<u32> {
        match self {
            AddressGeneratorMode::Negative(cst)
            | AddressGeneratorMode::Positive(cst)
            | AddressGeneratorMode::Predecrement(cst)
            | AddressGeneratorMode::Preincrement(cst)
            | AddressGeneratorMode::Postdecrement(cst)
            | AddressGeneratorMode::Postincrement(cst) => Some(*cst),
            _ => None,
        }
    }
}

impl Display for AddressGeneratorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressGeneratorMode::NegativeR(register) => write!(f, "*-R[{register}]"),
            AddressGeneratorMode::PositiveR(register) => write!(f, "*+R[{register}]"),
            AddressGeneratorMode::PredecrementR(register) => write!(f, "*--R[{register}]"),
            AddressGeneratorMode::PreincrementR(register) => write!(f, "*++R[{register}]"),
            AddressGeneratorMode::PostdecrementR(register) => write!(f, "*R--[{register}]"),
            AddressGeneratorMode::PostincrementR(register) => write!(f, "*R++[{register}]"),
            AddressGeneratorMode::Negative(cst) => write!(f, "*-R[{cst}]"),
            AddressGeneratorMode::Positive(cst) => write!(f, "*+R[{cst}]"),
            AddressGeneratorMode::Predecrement(cst) => write!(f, "*--R[{cst}]"),
            AddressGeneratorMode::Preincrement(cst) => write!(f, "*++R[{cst}]"),
            AddressGeneratorMode::Postdecrement(cst) => write!(f, "*R--[{cst}]"),
            AddressGeneratorMode::Postincrement(cst) => write!(f, "*R++[{cst}]"),
        }
    }
}

pub struct MemoryInstruction {
    instruction_data: InstructionData,
    instruction_type: MemoryInstructionType,
    data_size: DataSize,
    base_register: Register,
    mode: AddressGeneratorMode,
    side: bool,
    register: Register,
}

impl C64xInstruction for MemoryInstruction {
    fn new(input: &super::InstructionInput) -> Result<Self> {
        let format = [
            ParsingInstruction::Bit {
                name: String::from("p"),
            },
            ParsingInstruction::Bit {
                name: String::from("s"),
            },
            ParsingInstruction::Match { size: 2, value: 1 },
            ParsingInstruction::Unsigned {
                size: 3,
                name: String::from("op"),
            },
            ParsingInstruction::Bit {
                name: String::from("y"),
            },
            ParsingInstruction::Bit {
                name: String::from("op2"),
            },
            ParsingInstruction::Unsigned {
                size: 4,
                name: String::from("mode"),
            },
            ParsingInstruction::Unsigned {
                size: 5,
                name: String::from("offset"),
            },
            ParsingInstruction::Unsigned {
                size: 5,
                name: String::from("baseR"),
            },
            ParsingInstruction::Register {
                size: 5,
                name: String::from("register"),
            },
            ParsingInstruction::ConditionalOperation {
                name: String::from("creg"),
            },
        ];

        if let Ok(parsed_variables) = parse(input.opcode, &format) {
            let p_bit = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool()?;

            let op2 = ParsedVariable::try_get(&parsed_variables, "op2")?.get_bool()?;
            let (instruction_type, data_size) =
                match ParsedVariable::try_get(&parsed_variables, "op")?.get_u8()? {
                    0b111 if !op2 => (MemoryInstructionType::Store, DataSize::Word),
                    0b011 if !op2 => (MemoryInstructionType::Store, DataSize::Byte),
                    0b100 if op2 => (MemoryInstructionType::Store, DataSize::DoubleWord),
                    0b101 if !op2 => (MemoryInstructionType::Store, DataSize::HalfWord),
                    0b111 if op2 => (MemoryInstructionType::Store, DataSize::NonAlignedDoubleWord),
                    0b101 if op2 => (MemoryInstructionType::Store, DataSize::NonAlignedWord),
                    0b010 if !op2 => (MemoryInstructionType::Load, DataSize::Byte),
                    0b001 if !op2 => (MemoryInstructionType::Load, DataSize::ByteUnsigned),
                    0b110 if op2 => (MemoryInstructionType::Load, DataSize::DoubleWord),
                    0b100 if !op2 => (MemoryInstructionType::Load, DataSize::HalfWord),
                    0b000 if !op2 => (MemoryInstructionType::Load, DataSize::HalfWordUnsigned),
                    0b010 if op2 => (MemoryInstructionType::Load, DataSize::NonAlignedDoubleWord),
                    0b011 if op2 => (MemoryInstructionType::Load, DataSize::NonAlignedWord),
                    0b110 if !op2 => (MemoryInstructionType::Load, DataSize::Word),
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "Invalid memory instruction opcode",
                        ));
                    }
                };

            let side = ParsedVariable::try_get(&parsed_variables, "y")?.get_bool()?;
            let offset = ParsedVariable::try_get(&parsed_variables, "offset")?.get_u8()?;
            let mode = match ParsedVariable::try_get(&parsed_variables, "mode")?.get_u8()? {
                0b0000 => AddressGeneratorMode::Negative(offset as u32),
                0b0001 => AddressGeneratorMode::Positive(offset as u32),
                0b1000 => AddressGeneratorMode::Predecrement(offset as u32),
                0b1001 => AddressGeneratorMode::Preincrement(offset as u32),
                0b1010 => AddressGeneratorMode::Postdecrement(offset as u32),
                0b1011 => AddressGeneratorMode::Postincrement(offset as u32),
                0b0100 => AddressGeneratorMode::NegativeR(Register::from(offset, side)),
                0b0101 => AddressGeneratorMode::PositiveR(Register::from(offset, side)),
                0b1100 => AddressGeneratorMode::PredecrementR(Register::from(offset, side)),
                0b1101 => AddressGeneratorMode::PreincrementR(Register::from(offset, side)),
                0b1110 => AddressGeneratorMode::PostdecrementR(Register::from(offset, side)),
                0b1111 => AddressGeneratorMode::PostincrementR(Register::from(offset, side)),
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid memory instruction mode",
                    ));
                }
            };
            let base_register = Register::from(
                ParsedVariable::try_get(&parsed_variables, "baseR")?.get_u8()?,
                side,
            );
            let register =
                ParsedVariable::try_get(&parsed_variables, "register")?.get_register()?;

            return Ok(Self {
                instruction_type,
                data_size,
                mode,
                base_register,
                side,
                register,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: false,
                    p_bit,
                    ..Default::default()
                },
            });
        }

        Err(Error::new(
            ErrorKind::InvalidInput,
            "Not a memory load/store instruction",
        ))
    }

    fn instruction_clean(&self) -> String {
        let prefix = if self.instruction_type == MemoryInstructionType::Load {
            String::from("LD")
        } else {
            String::from("ST")
        };
        format!("{prefix}{}", self.data_size.to_short_string())
    }

    fn instruction(&self) -> String {
        format!(
            "{}.D{}T{}",
            self.instruction_clean(),
            if self.side { 2 } else { 1 },
            if self.register.side() { 2 } else { 1 },
        )
    }

    fn operands(&self) -> String {
        let mode = self
            .mode
            .to_string()
            .replace("R", self.base_register.to_string().as_str());
        let shift_by = match self.data_size {
            DataSize::Byte | DataSize::ByteUnsigned => 0,
            DataSize::HalfWord | DataSize::HalfWordUnsigned => 1,
            DataSize::Word | DataSize::NonAlignedWord => 2,
            DataSize::DoubleWord | DataSize::NonAlignedDoubleWord => 3, // NonAlignedDoubleWord varies based on sc field
        };
        let comment = {
            if let Some(register) = self.mode.get_register() {
                format!("({register} << {shift_by})")
            } else if let Some(constant) = self.mode.get_constant()
                && constant != 0
            {
                let result = constant << shift_by;
                format!("({constant} << {shift_by} = 0x{result:04X})")
            } else {
                String::new()
            }
        };
        if self.instruction_type == MemoryInstructionType::Load {
            format!("{mode}, {} {comment}", self.register)
        } else {
            format!("{}, {mode} {comment}", self.register)
        }
    }

    fn instruction_data(&self) -> &super::InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut super::InstructionData {
        &mut self.instruction_data
    }
}
