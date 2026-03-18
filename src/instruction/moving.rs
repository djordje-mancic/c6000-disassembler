use crate::instruction::{
    C6000Instruction, ConditionalOperation, InstructionData, Unit,
    formats::{
        FormatSymbol, d_unit,
        l_unit::{self, LX5_FORMAT},
        lsd::{LSDMVFR_FORMAT, LSDMVTO_FORMAT, LSDX1_FORMAT, LSDX1C_FORMAT},
        m_unit,
        s_unit::{self, MOVE_CONSTANT_FORMAT, SMVK8_FORMAT, SX1_FORMAT},
    },
    parser::{ParsingInstruction, parse},
    register::{ControlRegister, Register, RegisterFile},
};

pub struct MoveConstantInstruction {
    pub high: bool,
    pub constant: u32,
    pub destination: Register,
    pub unit: Unit,
    instruction_data: InstructionData,
}

impl C6000Instruction for MoveConstantInstruction {
    fn new(input: &super::InstructionInput) -> std::io::Result<Self> {
        let formats: [&[ParsingInstruction]; 3] = [
            &MOVE_CONSTANT_FORMAT,
            &l_unit::ONE_OR_TWO_SOURCES_FORMAT,
            &d_unit::ONE_OR_TWO_SOURCES_FORMAT,
        ];
        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, format) else {
                continue;
            };
            if format == &l_unit::ONE_OR_TWO_SOURCES_FORMAT {
                if parsed_variables.try_get_u8(FormatSymbol::Opfield)? != 0b0011010
                    || parsed_variables.try_get_u8(FormatSymbol::Source1)? != 0b00101
                {
                    continue;
                }
            } else if format == &d_unit::ONE_OR_TWO_SOURCES_FORMAT {
                if parsed_variables.try_get_u8(FormatSymbol::Opfield)? != 0
                    || parsed_variables.try_get_u8(FormatSymbol::Source2)? != 0
                {
                    continue;
                }
            }
            let p_bit = parsed_variables.try_get_bool(FormatSymbol::Parallel)?;
            let side = parsed_variables.try_get_bool(FormatSymbol::Side)?;
            let constant = {
                if format == &l_unit::ONE_OR_TWO_SOURCES_FORMAT {
                    parsed_variables.try_get_u32(FormatSymbol::Source2)?
                } else if format == &d_unit::ONE_OR_TWO_SOURCES_FORMAT {
                    parsed_variables.try_get_u32(FormatSymbol::Source1)?
                } else {
                    parsed_variables.try_get_u32(FormatSymbol::Constant(16))?
                }
            };
            let destination = parsed_variables
                .try_get_register(FormatSymbol::Destination)?
                .to_side(side);
            let high = {
                if format == &MOVE_CONSTANT_FORMAT {
                    parsed_variables.try_get_bool(FormatSymbol::MoveHigh)?
                } else {
                    false
                }
            };
            let conditional_operation = parsed_variables.try_get_conditional_operation()?;
            let unit = {
                if format == &l_unit::ONE_OR_TWO_SOURCES_FORMAT {
                    Unit::L
                } else if format == &d_unit::ONE_OR_TWO_SOURCES_FORMAT {
                    Unit::D
                } else {
                    Unit::S
                }
            };
            return Ok(Self {
                high,
                constant,
                destination,
                unit,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    conditional_operation,
                    p_bit,
                    ..Default::default()
                },
            });
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a Move Constant instruction: No matches found."),
        ))
    }

    fn new_compact(input: &super::InstructionInput) -> std::io::Result<Self> {
        let formats: [&[ParsingInstruction]; 4] =
            [&SMVK8_FORMAT, &LX5_FORMAT, &LSDX1C_FORMAT, &LSDX1_FORMAT];
        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, format) else {
                continue;
            };
            if format == &LSDX1_FORMAT {
                if parsed_variables.try_get_u8(FormatSymbol::Opfield)? & 0b110 != 0 {
                    continue;
                }
            }
            let side = parsed_variables.try_get_bool(FormatSymbol::Side)?;
            let constant = {
                if format == &SMVK8_FORMAT {
                    parsed_variables.try_get_u8(FormatSymbol::UnsignedConstant(8))?
                } else if format == &LX5_FORMAT {
                    parsed_variables.try_get_u8(FormatSymbol::SignedConstant(5))?
                } else if format == &LSDX1C_FORMAT {
                    parsed_variables.try_get_u8(FormatSymbol::UnsignedConstant(1))?
                } else if format == &LSDX1_FORMAT {
                    parsed_variables.try_get_u8(FormatSymbol::Opfield)? & 1
                } else {
                    break;
                }
            };
            let destination = (if format == &LSDX1_FORMAT {
                parsed_variables.try_get_register(FormatSymbol::Source)?
            } else {
                parsed_variables.try_get_register(FormatSymbol::Destination)?
            })
            .to_side(side);
            let unit = {
                if format == &SMVK8_FORMAT {
                    Unit::S
                } else if format == &LX5_FORMAT {
                    Unit::L
                } else {
                    parsed_variables.try_get_lsd_unit()?
                }
            };
            let conditional_operation = {
                if format == &LSDX1C_FORMAT {
                    match parsed_variables.try_get_u8(FormatSymbol::CC)? {
                        0b00 => Some(ConditionalOperation::NonZero(Register::A(0))),
                        0b01 => Some(ConditionalOperation::Zero(Register::A(0))),
                        0b10 => Some(ConditionalOperation::NonZero(Register::B(0))),
                        0b11 => Some(ConditionalOperation::Zero(Register::B(0))),
                        _ => break,
                    }
                } else {
                    None
                }
            };
            return Ok(Self {
                high: false,
                constant: constant as u32,
                destination,
                unit,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    conditional_operation,
                    compact: true,
                    ..Default::default()
                },
            });
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a Move Constant instruction: No matches found."),
        ))
    }

    fn instruction_clean(&self) -> String {
        if self.high {
            String::from("MVKH")
        } else {
            if self.constant == 0 {
                String::from("ZERO")
            } else {
                String::from("MVK")
            }
        }
    }

    fn instruction(&self) -> String {
        let mut value = self.instruction_clean();
        value += ".";
        let side = self.destination.side();
        value += self.unit.to_sided_string(side).as_str();
        value
    }

    fn operands(&self) -> String {
        if !self.high && self.constant == 0 {
            self.destination.to_string()
        } else {
            format!("0x{:04X}, {}", self.constant, self.destination.to_string())
        }
    }

    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}

pub struct MoveRegisterInstruction {
    pub source: RegisterFile,
    pub destination: RegisterFile,
    side: bool,
    pub delayed: bool,
    pub unit: Unit,
    instruction_data: InstructionData,
}

impl C6000Instruction for MoveRegisterInstruction {
    fn new(input: &super::InstructionInput) -> std::io::Result<Self> {
        let formats: [&[ParsingInstruction]; 5] = [
            &s_unit::ONE_OR_TWO_SOURCES_FORMAT,
            &d_unit::ONE_OR_TWO_SOURCES_FORMAT,
            &d_unit::EXTENDED_ONE_OR_TWO_SOURCES_FORMAT,
            &l_unit::ONE_OR_TWO_SOURCES_FORMAT,
            &m_unit::EXTENDED_UNARY_FORMAT,
        ];

        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, format) else {
                continue;
            };
            let op = parsed_variables.try_get_u8(FormatSymbol::Opfield)?;
            if (format == &s_unit::ONE_OR_TWO_SOURCES_FORMAT
                && op >> 1 != 0b000111
                && op != 0b000110)
                || (format == &l_unit::ONE_OR_TWO_SOURCES_FORMAT
                    && op != 0b0000010
                    && op != 0b1111110
                    && op != 0b0100000)
                || (format == &d_unit::ONE_OR_TWO_SOURCES_FORMAT && op != 0b010010)
                || (format == &d_unit::EXTENDED_ONE_OR_TWO_SOURCES_FORMAT && op != 0b0011)
                || (format == &m_unit::EXTENDED_UNARY_FORMAT && op != 0b11010)
            {
                continue;
            }
            let side = parsed_variables.try_get_bool(FormatSymbol::Side)?;
            let mvc = {
                if format == &s_unit::ONE_OR_TWO_SOURCES_FORMAT && op >> 1 == 0b000111 {
                    if side != true {
                        continue;
                    }
                    true
                } else {
                    false
                }
            };
            let crosspath = {
                if format == &d_unit::ONE_OR_TWO_SOURCES_FORMAT {
                    false
                } else {
                    parsed_variables.try_get_bool(FormatSymbol::Crosspath)?
                }
            };
            let delayed = {
                if format == &m_unit::EXTENDED_UNARY_FORMAT {
                    true
                } else {
                    false
                }
            };
            let src1 = {
                if format == &m_unit::EXTENDED_UNARY_FORMAT {
                    0
                } else {
                    parsed_variables.try_get_u8(FormatSymbol::Source1)?
                }
            };
            if src1 != 0 && !mvc {
                continue;
            }
            let src2 = parsed_variables.try_get_u8(FormatSymbol::Source2)?;
            let dst = parsed_variables.try_get_u8(FormatSymbol::Destination)?;

            let (source, destination) = {
                if mvc {
                    let control_to_register = op & 1 == 1;
                    let Some(control_register) = (if control_to_register {
                        ControlRegister::from(src2, src1)
                    } else {
                        ControlRegister::from(dst, src1)
                    }) else {
                        continue;
                    };
                    if control_to_register {
                        (
                            RegisterFile::Control(control_register),
                            RegisterFile::GeneralPurpose(Register::from(dst, side)),
                        )
                    } else {
                        (
                            RegisterFile::GeneralPurpose(Register::from(src2, side ^ crosspath)),
                            RegisterFile::Control(control_register),
                        )
                    }
                } else {
                    let mut src_register = Register::from(src2, side ^ crosspath);
                    let mut dst_register = Register::from(dst, side);
                    if format == &l_unit::ONE_OR_TWO_SOURCES_FORMAT && op == 0b0100000 {
                        src_register = src_register.to_pair();
                        dst_register = dst_register.to_pair();
                    }
                    (
                        RegisterFile::GeneralPurpose(src_register),
                        RegisterFile::GeneralPurpose(dst_register),
                    )
                }
            };

            let p_bit = parsed_variables.try_get_bool(FormatSymbol::Parallel)?;
            let conditional_operation = parsed_variables.try_get_conditional_operation()?;
            let unit = {
                if format == &s_unit::ONE_OR_TWO_SOURCES_FORMAT {
                    Unit::S
                } else if format == &d_unit::ONE_OR_TWO_SOURCES_FORMAT
                    || format == &d_unit::EXTENDED_ONE_OR_TWO_SOURCES_FORMAT
                {
                    Unit::D
                } else if format == &l_unit::ONE_OR_TWO_SOURCES_FORMAT {
                    Unit::L
                } else {
                    Unit::M
                }
            };

            return Ok(Self {
                source,
                destination,
                unit,
                side,
                delayed,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    conditional_operation,
                    p_bit,
                    ..Default::default()
                },
            });
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a Move Register instruction: No matches found."),
        ))
    }

    fn new_compact(input: &super::InstructionInput) -> std::io::Result<Self> {
        let formats: [&[ParsingInstruction]; 3] = [&LSDMVTO_FORMAT, &LSDMVFR_FORMAT, &SX1_FORMAT];

        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, format) else {
                continue;
            };
            let side = parsed_variables.try_get_bool(FormatSymbol::Side)?;
            if format == &SX1_FORMAT {
                if parsed_variables.try_get_u8(FormatSymbol::Opfield)? != 0b110 || side != true {
                    continue;
                }
            }
            let unit = {
                if format == &SX1_FORMAT {
                    Unit::S
                } else {
                    parsed_variables.try_get_lsd_unit()?
                }
            };
            let crosspath = {
                if format == &SX1_FORMAT {
                    false
                } else {
                    parsed_variables.try_get_bool(FormatSymbol::Crosspath)?
                }
            };
            let source = RegisterFile::GeneralPurpose(
                parsed_variables
                    .try_get_register(FormatSymbol::Source2)?
                    .to_side(side ^ crosspath),
            );
            let destination = {
                if format == &SX1_FORMAT {
                    RegisterFile::Control(ControlRegister::ILC)
                } else {
                    RegisterFile::GeneralPurpose(
                        parsed_variables
                            .try_get_register(FormatSymbol::Destination)?
                            .to_side(side),
                    )
                }
            };
            return Ok(Self {
                source,
                destination,
                side,
                delayed: false,
                unit,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: true,
                    ..Default::default()
                },
            });
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Not a Move Register instruction: No matches found."),
        ))
    }

    fn instruction_clean(&self) -> String {
        if self.destination.side() == None || self.source.side() == None {
            String::from("MVC")
        } else if self.delayed {
            String::from("MVD")
        } else {
            String::from("MV")
        }
    }

    fn instruction(&self) -> String {
        let mut value = format!(
            "{}.{}",
            self.instruction_clean(),
            self.unit.to_sided_string(self.side)
        );

        if self.destination.side() == Some(!self.side) || self.source.side() == Some(!self.side) {
            value += "X";
        }
        value
    }

    fn operands(&self) -> String {
        format!(
            "{}, {}",
            self.source.to_string(),
            self.destination.to_string()
        )
    }

    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}
