use std::{
    fmt::Display,
    io::{Error, ErrorKind, Result},
};

use crate::instruction::{
    C6000Instruction, DataSize, InstructionData,
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

impl C6000Instruction for MemoryInstruction {
    fn new(input: &super::InstructionInput) -> Result<Self> {
        let formats = [
            vec![
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
            ],
            vec![
                ParsingInstruction::Bit {
                    name: String::from("p"),
                },
                ParsingInstruction::Bit {
                    name: String::from("s"),
                },
                ParsingInstruction::Match {
                    size: 2,
                    value: 0b11,
                },
                ParsingInstruction::Unsigned {
                    size: 3,
                    name: String::from("op"),
                },
                ParsingInstruction::Bit {
                    name: String::from("y"),
                },
                ParsingInstruction::Unsigned {
                    size: 15,
                    name: String::from("cst"),
                },
                ParsingInstruction::Register {
                    size: 5,
                    name: String::from("register"),
                },
                ParsingInstruction::ConditionalOperation {
                    name: String::from("creg"),
                },
            ],
        ];

        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, &format) else {
                continue;
            };
            let p_bit = ParsedVariable::try_get(&parsed_variables, "p")?.get_bool()?;

            let op2 = {
                if let Ok(var) = ParsedVariable::try_get(&parsed_variables, "op2") {
                    var.get_bool()?
                } else {
                    false
                }
            };
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
            let mode = {
                if let Ok(var) = ParsedVariable::try_get(&parsed_variables, "mode") {
                    let offset = ParsedVariable::try_get(&parsed_variables, "offset")?.get_u8()?;
                    let res = match var.get_u8()? {
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
                        0b1110 => {
                            AddressGeneratorMode::PostdecrementR(Register::from(offset, side))
                        }
                        0b1111 => {
                            AddressGeneratorMode::PostincrementR(Register::from(offset, side))
                        }
                        _ => {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                "Invalid memory instruction mode",
                            ));
                        }
                    };
                    res
                } else {
                    let cst = ParsedVariable::try_get(&parsed_variables, "cst")?.get_u32()?;
                    AddressGeneratorMode::Positive(cst)
                }
            };

            let base_register = {
                if let Ok(var) = ParsedVariable::try_get(&parsed_variables, "baseR") {
                    Register::from(var.get_u8()?, side)
                } else {
                    if side {
                        Register::B(15)
                    } else {
                        Register::B(14)
                    }
                }
            };
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

    fn new_compact(input: &super::InstructionInput) -> Result<Self> {
        let Some(fphead) = &input.fphead else {
            return Err(Error::new(ErrorKind::InvalidInput, "No fphead"));
        };

        let formats = [
            (
                "Doff4",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b10,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("load"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("register"),
                        size: 3,
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("ptr"),
                        size: 2,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("sz"),
                    },
                    ParsingInstruction::Match { size: 1, value: 0 },
                    ParsingInstruction::Unsigned {
                        name: String::from("cst3"),
                        size: 1,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("t"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("cst20"),
                        size: 3,
                    },
                ],
            ),
            (
                "Doff4DW",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b10,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("load"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("na"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("register"),
                        size: 2,
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("ptr"),
                        size: 2,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("sz"),
                    },
                    ParsingInstruction::Match { size: 1, value: 0 },
                    ParsingInstruction::Unsigned {
                        name: String::from("cst3"),
                        size: 1,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("t"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("cst20"),
                        size: 3,
                    },
                ],
            ),
            (
                "Dind",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b10,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("load"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("register"),
                        size: 3,
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("ptr"),
                        size: 2,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("sz"),
                    },
                    ParsingInstruction::Match { size: 2, value: 1 },
                    ParsingInstruction::Bit {
                        name: String::from("t"),
                    },
                    ParsingInstruction::Register {
                        name: String::from("src"),
                        size: 3,
                    },
                ],
            ),
            (
                "DindDW",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b10,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("load"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("na"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("register"),
                        size: 2,
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("ptr"),
                        size: 2,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("sz"),
                    },
                    ParsingInstruction::Match { size: 2, value: 1 },
                    ParsingInstruction::Bit {
                        name: String::from("t"),
                    },
                    ParsingInstruction::Register {
                        name: String::from("src"),
                        size: 3,
                    },
                ],
            ),
            (
                "Dincdec",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b10,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("load"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("register"),
                        size: 3,
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("ptr"),
                        size: 2,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("sz"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b11,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("t"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("cst0"),
                        size: 1,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("dec"),
                    },
                    ParsingInstruction::Match { size: 1, value: 0 },
                ],
            ),
            (
                "DincdecDW",
                vec![
                    ParsingInstruction::Bit {
                        name: String::from("s"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b10,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("load"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("na"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("register"),
                        size: 2,
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("ptr"),
                        size: 2,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("sz"),
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b11,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("t"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("cst0"),
                        size: 1,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("dec"),
                    },
                    ParsingInstruction::Match { size: 1, value: 0 },
                ],
            ),
            (
                "Dstk",
                vec![
                    ParsingInstruction::BitMatch {
                        name: String::from("s"),
                        value: true,
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b10,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("load"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("register"),
                        size: 3,
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("cst42"),
                        size: 3,
                    },
                    ParsingInstruction::Match {
                        size: 2,
                        value: 0b11,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("t"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("cst10"),
                        size: 2,
                    },
                    ParsingInstruction::Match { size: 1, value: 1 },
                ],
            ),
            (
                "Dpp",
                vec![
                    ParsingInstruction::BitMatch {
                        name: String::from("s"),
                        value: true,
                    },
                    ParsingInstruction::Match {
                        size: 6,
                        value: 0b111011,
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("register"),
                        size: 4,
                    },
                    ParsingInstruction::Match { size: 1, value: 0 },
                    ParsingInstruction::Bit {
                        name: String::from("t"),
                    },
                    ParsingInstruction::Unsigned {
                        name: String::from("cst0"),
                        size: 1,
                    },
                    ParsingInstruction::Bit {
                        name: String::from("load"),
                    },
                    ParsingInstruction::Bit {
                        name: String::from("dw"),
                    },
                ],
            ),
        ];
        for (name, format) in formats {
            let Ok(parsed_variables) = parse(input.opcode, format.as_slice()) else {
                continue;
            };
            let side = ParsedVariable::try_get(&parsed_variables, "s")?.get_bool()?;
            let t = ParsedVariable::try_get(&parsed_variables, "t")?.get_bool()?;

            let instruction_type = {
                if ParsedVariable::try_get(&parsed_variables, "load")?.get_bool()? {
                    MemoryInstructionType::Load
                } else {
                    MemoryInstructionType::Store
                }
            };

            let base_register = {
                if name == "Dstk" || name == "Dpp" {
                    Register::B(15)
                } else {
                    let ptr = ParsedVariable::try_get(&parsed_variables, "ptr")?.get_u8()?;
                    Register::from(ptr + 4, side)
                }
            };

            let mode = {
                if name.starts_with("Doff4") {
                    let cst20 = ParsedVariable::try_get(&parsed_variables, "cst20")?.get_u8()?;
                    let cst3 = ParsedVariable::try_get(&parsed_variables, "cst3")?.get_u8()?;
                    let cst = cst20 + (cst3 << 3);
                    AddressGeneratorMode::Positive(cst as u32)
                } else if name.starts_with("Dind") {
                    let src = ParsedVariable::try_get(&parsed_variables, "src")?.get_register()?;
                    AddressGeneratorMode::PositiveR(src)
                } else if name.starts_with("Dincdec") {
                    let cst = ParsedVariable::try_get(&parsed_variables, "cst0")?.get_u8()? + 1;
                    let dec = ParsedVariable::try_get(&parsed_variables, "dec")?.get_bool()?;
                    if dec {
                        AddressGeneratorMode::Predecrement(cst as u32)
                    } else {
                        AddressGeneratorMode::Postincrement(cst as u32)
                    }
                } else if name == "Dstk" {
                    let cst10 = ParsedVariable::try_get(&parsed_variables, "cst10")?.get_u8()?;
                    let cst42 = ParsedVariable::try_get(&parsed_variables, "cst42")?.get_u8()?;
                    let cst = cst10 + (cst42 << 2);
                    AddressGeneratorMode::Positive(cst as u32)
                } else if name == "Dpp" {
                    let cst = ParsedVariable::try_get(&parsed_variables, "cst0")?.get_u8()? + 1;
                    if instruction_type == MemoryInstructionType::Load {
                        AddressGeneratorMode::Preincrement(cst as u32)
                    } else {
                        AddressGeneratorMode::Postdecrement(cst as u32)
                    }
                } else {
                    AddressGeneratorMode::Positive(0)
                }
            };

            let data_size = {
                if name == "Dstk" {
                    DataSize::Word
                } else if name == "Dpp" {
                    let dw = ParsedVariable::try_get(&parsed_variables, "dw")?.get_bool()?;
                    if dw {
                        DataSize::DoubleWord
                    } else {
                        DataSize::Word
                    }
                } else {
                    let sz = ParsedVariable::try_get(&parsed_variables, "sz")?.get_bool()?;
                    if sz {
                        if name.ends_with("DW") {
                            continue;
                        }
                        fphead.secondary_data_size
                    } else {
                        if name.ends_with("DW") {
                            if fphead.primary_data_size != DataSize::DoubleWord {
                                continue;
                            }
                            let na =
                                ParsedVariable::try_get(&parsed_variables, "na")?.get_bool()?;
                            if na {
                                DataSize::NonAlignedDoubleWord
                            } else {
                                DataSize::DoubleWord
                            }
                        } else {
                            if fphead.primary_data_size != DataSize::Word {
                                continue;
                            }
                            DataSize::Word
                        }
                    }
                }
            };

            let register = {
                let reg_value = ParsedVariable::try_get(&parsed_variables, "register")?.get_u8()?;
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
