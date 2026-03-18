use std::io::{self, Error, ErrorKind};

use crate::instruction::{
    C6000Instruction, ConditionalOperation, InstructionData,
    formats::{
        FormatSymbol,
        s_unit::{
            BDEC_BPOS_POINTER_FORMAT, BRANCH_DISPLACEMENT_FORMAT, BRANCH_DISPLACEMENT_NOP_FORMAT,
            BRANCH_POINTER_FORMAT, BRANCH_REGISTER_FORMAT, BRANCH_REGISTER_NOP_FORMAT, SBS7_FORMAT,
            SBS7C_FORMAT, SBU8_FORMAT, SBU8C_FORMAT, SCS10_FORMAT, SX1B_FORMAT,
        },
    },
    parser::{ParsingInstruction, parse},
    register::{ControlRegister, Register},
};

pub enum BranchUsing {
    Displacement(i32),
    Register(Register),
    Pointer(ControlRegister),
}

pub enum AdditionalOperation {
    Predication(Register),
    PredicationAndDecrement(Register),
}

pub struct BranchInstruction {
    instruction_data: InstructionData,
    pub branch_using: BranchUsing,
    pub side: bool,
    pce1_address: u32,
    pub nop_count: u8,
    pub additional_operation: Option<AdditionalOperation>,
}

impl BranchInstruction {
    pub fn calculate_displacement_address(&self) -> io::Result<u32> {
        match self.branch_using {
            BranchUsing::Displacement(displacement) => {
                let displacement_abs = displacement.unsigned_abs();
                if self.pce1_address == 0 {
                    return Err(Error::other("PCE1 = 0"));
                };
                let branch_address = {
                    if displacement.is_positive() {
                        self.pce1_address + displacement_abs
                    } else {
                        self.pce1_address - displacement_abs
                    }
                };
                return Ok(branch_address);
            }
            _ => return Err(Error::other("Not displacement")),
        }
    }
}

impl C6000Instruction for BranchInstruction {
    fn new(input: &super::InstructionInput) -> std::io::Result<Self> {
        let formats: [&[ParsingInstruction]; 6] = [
            &BRANCH_DISPLACEMENT_FORMAT,
            &BRANCH_REGISTER_FORMAT,
            &BRANCH_POINTER_FORMAT,
            &BRANCH_DISPLACEMENT_NOP_FORMAT,
            &BRANCH_REGISTER_NOP_FORMAT,
            &BDEC_BPOS_POINTER_FORMAT,
        ];
        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, format) else {
                continue;
            };
            let p_bit = parsed_variables.try_get_bool(FormatSymbol::Parallel)?;
            let side = {
                if format == &BRANCH_POINTER_FORMAT || format == &BRANCH_REGISTER_NOP_FORMAT {
                    true
                } else {
                    parsed_variables.try_get_bool(FormatSymbol::Side)?
                }
            };
            let conditional_operation = parsed_variables.try_get_conditional_operation()?;
            let nop_count = {
                if format == &BRANCH_DISPLACEMENT_NOP_FORMAT
                    || format == &BRANCH_REGISTER_NOP_FORMAT
                {
                    parsed_variables.try_get_u8(FormatSymbol::Source1)?
                } else if format == &BRANCH_DISPLACEMENT_FORMAT
                    && conditional_operation == Some(ConditionalOperation::ReservedLow)
                {
                    5
                } else {
                    0
                }
            };
            let additional_operation = {
                if format == &BDEC_BPOS_POINTER_FORMAT {
                    let dst = parsed_variables
                        .try_get_register(FormatSymbol::Destination)?
                        .to_side(side);
                    if parsed_variables.try_get_bool(FormatSymbol::IsBDec)? {
                        Some(AdditionalOperation::PredicationAndDecrement(dst))
                    } else {
                        Some(AdditionalOperation::Predication(dst))
                    }
                } else {
                    None
                }
            };
            let branch_using = {
                if format == &BRANCH_DISPLACEMENT_FORMAT {
                    BranchUsing::Displacement(
                        parsed_variables.try_get_i32(FormatSymbol::Constant(21))? << 2,
                    )
                } else if format == &BRANCH_DISPLACEMENT_NOP_FORMAT {
                    BranchUsing::Displacement(
                        parsed_variables.try_get_i32(FormatSymbol::Source2)? << {
                            if input.fphead.is_some() { 1 } else { 2 }
                        },
                    )
                } else if format == &BRANCH_REGISTER_FORMAT || format == &BRANCH_REGISTER_NOP_FORMAT
                {
                    let crosspath = parsed_variables.try_get_bool(FormatSymbol::Crosspath)?;
                    BranchUsing::Register(
                        parsed_variables
                            .try_get_register(FormatSymbol::Source2)?
                            .to_side(side ^ crosspath),
                    )
                } else if format == &BRANCH_POINTER_FORMAT {
                    let opcode = parsed_variables.try_get_u8(FormatSymbol::Opfield)?;
                    match opcode {
                        0b110 => BranchUsing::Pointer(ControlRegister::IRP),
                        0b111 => BranchUsing::Pointer(ControlRegister::NRP),
                        _ => continue,
                    }
                } else if format == &BDEC_BPOS_POINTER_FORMAT {
                    BranchUsing::Displacement(
                        parsed_variables.try_get_i32(FormatSymbol::Source)? << 2,
                    )
                } else {
                    continue;
                }
            };
            return Ok(Self {
                side,
                branch_using,
                pce1_address: input.pce1_address,
                nop_count,
                additional_operation,
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: false,
                    conditional_operation,
                    p_bit,
                    ..Default::default()
                },
            });
        }
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Not a branch instruction",
        ))
    }

    fn new_compact(input: &super::InstructionInput) -> std::io::Result<Self> {
        let formats: [&[ParsingInstruction]; 6] = [
            &SBS7_FORMAT,
            &SBU8_FORMAT,
            &SCS10_FORMAT,
            &SBS7C_FORMAT,
            &SBU8C_FORMAT,
            &SX1B_FORMAT,
        ];

        for format in formats {
            let Ok(parsed_variables) = parse(input.opcode, format) else {
                continue;
            };
            let side = parsed_variables.try_get_bool(FormatSymbol::Side)?;
            let branch_using = {
                if format == &SX1B_FORMAT {
                    BranchUsing::Register(Register::B(
                        parsed_variables.try_get_u8(FormatSymbol::Source2)?,
                    ))
                } else if format == &SCS10_FORMAT {
                    BranchUsing::Displacement(
                        parsed_variables.try_get_i32(FormatSymbol::SignedConstant(10))? << 2,
                    )
                } else if format == &SBS7_FORMAT || format == &SBS7C_FORMAT {
                    BranchUsing::Displacement(
                        parsed_variables.try_get_i32(FormatSymbol::SignedConstant(7))? << 1,
                    )
                } else if format == &SBU8_FORMAT || format == &SBU8C_FORMAT {
                    BranchUsing::Displacement(
                        (parsed_variables.try_get_u32(FormatSymbol::UnsignedConstant(8))? << 1)
                            as i32,
                    )
                } else {
                    break;
                }
            };
            let nop_count = {
                if format == &SBS7_FORMAT || format == &SBS7C_FORMAT || format == &SX1B_FORMAT {
                    let nop = parsed_variables.try_get_u8(FormatSymbol::N3)?;
                    if nop > 5 {
                        continue;
                    }
                    nop
                } else {
                    5
                }
            };
            let conditional_operation = {
                if format == &SCS10_FORMAT {
                    Some(ConditionalOperation::ReservedLow)
                } else if format == &SBS7C_FORMAT || format == &SBU8C_FORMAT {
                    if parsed_variables.try_get_bool(FormatSymbol::Zero)? {
                        Some(ConditionalOperation::Zero(Register::from(0, side)))
                    } else {
                        Some(ConditionalOperation::NonZero(Register::from(0, side)))
                    }
                } else {
                    None
                }
            };
            return Ok(Self {
                instruction_data: InstructionData {
                    opcode: input.opcode,
                    compact: true,
                    conditional_operation,
                    ..Default::default()
                },
                branch_using,
                side,
                pce1_address: input.pce1_address,
                nop_count,
                additional_operation: None,
            });
        }

        Err(Error::new(
            ErrorKind::InvalidInput,
            "Not a branch instruction",
        ))
    }

    fn instruction_clean(&self) -> String {
        if let Some(co) = self.conditional_operation()
            && co == ConditionalOperation::ReservedLow
        {
            String::from("CALLP")
        } else {
            if self.nop_count > 0 {
                String::from("BNOP")
            } else if let Some(operation) = &self.additional_operation {
                match operation {
                    AdditionalOperation::Predication(_) => String::from("BPOS"),
                    AdditionalOperation::PredicationAndDecrement(_) => String::from("BDEC"),
                }
            } else {
                String::from("B")
            }
        }
    }

    fn instruction(&self) -> String {
        let unit_num = if self.side { 2 } else { 1 };
        let mut instruction = format!("{}.S{unit_num}", self.instruction_clean());
        if let BranchUsing::Register(register) = self.branch_using
            && register.side() != self.side
        {
            instruction += "X";
        }
        instruction
    }

    fn operands(&self) -> String {
        let operands = match self.branch_using {
            BranchUsing::Displacement(displacement) => {
                let displacement_abs = displacement.unsigned_abs();
                let address_result = self.calculate_displacement_address();
                let Ok(branch_address) = self.calculate_displacement_address() else {
                    return address_result
                        .map_err(|e| format!("ERROR {e}"))
                        .unwrap_err();
                };
                let predication_string = {
                    if let Some(operation) = &self.additional_operation {
                        match operation {
                            AdditionalOperation::Predication(register)
                            | AdditionalOperation::PredicationAndDecrement(register) => {
                                format!(", {register}")
                            }
                        }
                    } else {
                        String::new()
                    }
                };
                format!(
                    "0x{branch_address:08X} (PCE1{}0x{displacement_abs:08X}){predication_string}",
                    if displacement.is_positive() { "+" } else { "-" }
                )
            }
            BranchUsing::Register(register) => register.to_string(),
            BranchUsing::Pointer(register) => register.to_string(),
        };

        if let Some(co) = self.conditional_operation()
            && co == ConditionalOperation::ReservedLow
        {
            format!("{operands}, {}", Register::from(3, self.side).to_string())
        } else if self.nop_count > 0 {
            format!("{operands}, {}", self.nop_count)
        } else {
            operands
        }
    }

    fn instruction_data(&self) -> &InstructionData {
        &self.instruction_data
    }

    fn instruction_data_mut(&mut self) -> &mut InstructionData {
        &mut self.instruction_data
    }
}
