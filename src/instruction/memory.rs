use std::{
    fmt::Display,
    io::{Error, ErrorKind, Result},
};

use crate::instruction::{
    C6000Instruction, DataSize, InstructionData,
    formats::{
        FormatSymbol,
        d_unit::{
            DDEC_FORMAT, DDECDW_FORMAT, DINC_FORMAT, DINCDW_FORMAT, DIND_FORMAT, DINDDW_FORMAT,
            DOFF4_FORMAT, DOFF4DW_FORMAT, DPP_FORMAT, DSTK_FORMAT, LOAD_STORE_BASIC_FORMAT,
            LOAD_STORE_LONG_FORMAT, LOAD_STORE_NDW_FORMAT,
        },
    },
    parser::{ParsingInstruction, parse},
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

impl C6000Instruction for MemoryInstruction {
    fn new(input: &super::InstructionInput) -> Result<Self> {
        let formats: [&[ParsingInstruction]; 3] = [
            &LOAD_STORE_BASIC_FORMAT,
            &LOAD_STORE_LONG_FORMAT,
            &LOAD_STORE_NDW_FORMAT,
        ];

        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, &format) else {
                continue;
            };
            let p_bit = parsed_variables.try_get_bool(FormatSymbol::Parallel)?;

            let r = {
                if format == &LOAD_STORE_LONG_FORMAT {
                    false
                } else {
                    parsed_variables.try_get_bool(FormatSymbol::LoadStoreR)?
                }
            };
            let (instruction_type, data_size) =
                match parsed_variables.try_get_u8(FormatSymbol::Opfield)? {
                    0b111 if !r => (MemoryInstructionType::Store, DataSize::Word),
                    0b011 if !r => (MemoryInstructionType::Store, DataSize::Byte),
                    0b100 if r => (MemoryInstructionType::Store, DataSize::DoubleWord),
                    0b101 if !r => (MemoryInstructionType::Store, DataSize::HalfWord),
                    0b111 if r => (MemoryInstructionType::Store, DataSize::NonAlignedDoubleWord),
                    0b101 if r => (MemoryInstructionType::Store, DataSize::NonAlignedWord),
                    0b010 if !r => (MemoryInstructionType::Load, DataSize::Byte),
                    0b001 if !r => (MemoryInstructionType::Load, DataSize::ByteUnsigned),
                    0b110 if r => (MemoryInstructionType::Load, DataSize::DoubleWord),
                    0b100 if !r => (MemoryInstructionType::Load, DataSize::HalfWord),
                    0b000 if !r => (MemoryInstructionType::Load, DataSize::HalfWordUnsigned),
                    0b010 if r => (MemoryInstructionType::Load, DataSize::NonAlignedDoubleWord),
                    0b011 if r => (MemoryInstructionType::Load, DataSize::NonAlignedWord),
                    0b110 if !r => (MemoryInstructionType::Load, DataSize::Word),
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            "Invalid memory instruction opcode",
                        ));
                    }
                };

            let side = parsed_variables.try_get_bool(FormatSymbol::DUnitSide)?;
            let register_side = parsed_variables.try_get_bool(FormatSymbol::Side)?;
            let mode = {
                let offset = parsed_variables.try_get_u32(FormatSymbol::RegisterOffset)?;
                if format == &LOAD_STORE_LONG_FORMAT {
                    AddressGeneratorMode::Positive(offset)
                } else {
                    let res = match parsed_variables.try_get_u8(FormatSymbol::AddressingMode)? {
                        0b0000 => AddressGeneratorMode::Negative(offset),
                        0b0001 => AddressGeneratorMode::Positive(offset),
                        0b1000 => AddressGeneratorMode::Predecrement(offset),
                        0b1001 => AddressGeneratorMode::Preincrement(offset),
                        0b1010 => AddressGeneratorMode::Postdecrement(offset),
                        0b1011 => AddressGeneratorMode::Postincrement(offset),
                        0b0100 => {
                            AddressGeneratorMode::NegativeR(Register::from(offset as u8, side))
                        }
                        0b0101 => {
                            AddressGeneratorMode::PositiveR(Register::from(offset as u8, side))
                        }
                        0b1100 => {
                            AddressGeneratorMode::PredecrementR(Register::from(offset as u8, side))
                        }
                        0b1101 => {
                            AddressGeneratorMode::PreincrementR(Register::from(offset as u8, side))
                        }
                        0b1110 => {
                            AddressGeneratorMode::PostdecrementR(Register::from(offset as u8, side))
                        }
                        0b1111 => {
                            AddressGeneratorMode::PostincrementR(Register::from(offset as u8, side))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                "Invalid memory instruction mode",
                            ));
                        }
                    };
                    res
                }
            };

            let base_register = {
                if format == &LOAD_STORE_LONG_FORMAT {
                    if side {
                        Register::B(15)
                    } else {
                        Register::B(14)
                    }
                } else {
                    parsed_variables
                        .try_get_register(FormatSymbol::BaseRegister)?
                        .to_side(side)
                }
            };
            let register = parsed_variables
                .try_get_register(FormatSymbol::SourceOrDestination)?
                .to_side(register_side);

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

    fn new_compact(input: &super::InstructionInput) -> Result<Self> {
        let Some(fphead) = &input.fphead else {
            return Err(Error::new(ErrorKind::InvalidInput, "No fphead"));
        };

        let formats: [&[ParsingInstruction]; 10] = [
            &DOFF4_FORMAT,
            &DOFF4DW_FORMAT,
            &DIND_FORMAT,
            &DINDDW_FORMAT,
            &DINC_FORMAT,
            &DINCDW_FORMAT,
            &DDEC_FORMAT,
            &DDECDW_FORMAT,
            &DSTK_FORMAT,
            &DPP_FORMAT,
        ];
        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, format) else {
                continue;
            };
            if (format == &DOFF4DW_FORMAT
                || format == &DINDDW_FORMAT
                || format == &DINCDW_FORMAT
                || format == &DDECDW_FORMAT)
                && fphead.primary_data_size != DataSize::DoubleWord
            {
                continue;
            }

            let side = parsed_variables.try_get_bool(FormatSymbol::Side)?;
            let t = parsed_variables.try_get_bool(FormatSymbol::Side2)?;

            let instruction_type = {
                if parsed_variables.try_get_bool(FormatSymbol::IsLoad)? {
                    MemoryInstructionType::Load
                } else {
                    MemoryInstructionType::Store
                }
            };

            let base_register = {
                if format == &DPP_FORMAT || format == &DSTK_FORMAT {
                    Register::B(15)
                } else {
                    Register::from(
                        parsed_variables.try_get_u8(FormatSymbol::Pointer)? | 0b100,
                        side,
                    )
                }
            };

            let mode = {
                if format == &DOFF4_FORMAT || format == &DOFF4DW_FORMAT {
                    AddressGeneratorMode::Positive(
                        parsed_variables.try_get_u32(FormatSymbol::UnsignedConstant(4))?,
                    )
                } else if format == &DIND_FORMAT || format == &DINDDW_FORMAT {
                    AddressGeneratorMode::PositiveR(
                        parsed_variables
                            .try_get_register(FormatSymbol::Source1)?
                            .to_side(side),
                    )
                } else if format == &DINC_FORMAT || format == &DINCDW_FORMAT {
                    AddressGeneratorMode::Postincrement(
                        parsed_variables.try_get_u32(FormatSymbol::UnsignedConstant(2))? + 1,
                    )
                } else if format == &DDEC_FORMAT || format == &DDECDW_FORMAT {
                    AddressGeneratorMode::Predecrement(
                        parsed_variables.try_get_u32(FormatSymbol::UnsignedConstant(2))? + 1,
                    )
                } else if format == &DSTK_FORMAT {
                    AddressGeneratorMode::Positive(
                        parsed_variables.try_get_u32(FormatSymbol::UnsignedConstant(5))?,
                    )
                } else if format == &DPP_FORMAT {
                    let cst = parsed_variables.try_get_u32(FormatSymbol::UnsignedConstant(2))? + 1;
                    if instruction_type == MemoryInstructionType::Load {
                        AddressGeneratorMode::Preincrement(cst)
                    } else {
                        AddressGeneratorMode::Postdecrement(cst)
                    }
                } else {
                    break;
                }
            };

            let data_size = {
                if format == &DSTK_FORMAT {
                    DataSize::Word
                } else if format == &DPP_FORMAT {
                    if parsed_variables.try_get_bool(FormatSymbol::IsDoubleWord)? {
                        DataSize::DoubleWord
                    } else {
                        DataSize::Word
                    }
                } else {
                    if parsed_variables.try_get_bool(FormatSymbol::DataSize)? {
                        match fphead.secondary_data_size {
                            DataSize::ByteUnsigned
                                if instruction_type == MemoryInstructionType::Store =>
                            {
                                DataSize::Byte
                            }
                            DataSize::HalfWordUnsigned
                                if instruction_type == MemoryInstructionType::Store =>
                            {
                                DataSize::HalfWord
                            }
                            other => other,
                        }
                    } else {
                        if fphead.primary_data_size == DataSize::Word {
                            DataSize::Word
                        } else {
                            if parsed_variables.try_get_bool(FormatSymbol::NonAligned)? {
                                DataSize::NonAlignedDoubleWord
                            } else {
                                DataSize::DoubleWord
                            }
                        }
                    }
                }
            };

            let register = {
                let reg_value = parsed_variables.try_get_u8(FormatSymbol::SourceOrDestination)?;
                if data_size == DataSize::DoubleWord || data_size == DataSize::NonAlignedDoubleWord
                {
                    Register::from_pair(reg_value, t)
                } else {
                    Register::from(reg_value, t)
                }
            };

            return Ok(Self {
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: true,
                    ..Default::default()
                },
                instruction_type,
                data_size,
                base_register,
                mode,
                side,
                register,
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
