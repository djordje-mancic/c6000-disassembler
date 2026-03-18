use std::{
    cmp::max,
    collections::HashMap,
    io::{Error, ErrorKind, Result},
};

use crate::instruction::{ConditionalOperation, Unit, formats::FormatSymbol, register::Register};

pub fn parse(opcode: u32, format: &[ParsingInstruction]) -> Result<ParsingResult> {
    let mut result = ParsingResult::new();
    let mut temp_opcode = opcode;

    for instruction in format {
        match instruction {
            ParsingInstruction::Match { size, value } => {
                let masked_value = read_u32(&mut temp_opcode, *size);
                if masked_value != *value {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "Opcode does not match instruction format (got {masked_value:b} instead of {value:b})"
                        ),
                    ));
                }
            }
            ParsingInstruction::Bit { symbol } | ParsingInstruction::BitMatch { symbol, .. } => {
                let read_value = read_bool(&mut temp_opcode);
                if let ParsingInstruction::BitMatch { value, .. } = instruction {
                    if read_value != *value {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!(
                                "Opcode does not match instruction format ({symbol:?} is {read_value} instead of {value})"
                            ),
                        ));
                    }
                }
                result.add(*symbol, ParsedVariable::Bool(read_value));
            }
            ParsingInstruction::BitArray { size, symbol } => {
                let mut value = Vec::<bool>::new();
                for _ in 0..*size {
                    value.push(read_bool(&mut temp_opcode));
                }
                result.add(*symbol, ParsedVariable::BoolVec(value));
            }
            ParsingInstruction::Bitfield { size, symbol }
            | ParsingInstruction::BitfieldChunk { size, symbol, .. } => {
                let value = read_u32(&mut temp_opcode, *size);
                let parsed_variable = {
                    if let ParsingInstruction::BitfieldChunk { index, .. } = instruction {
                        if result.variables.get(symbol).is_none() {
                            result
                                .variables
                                .insert(*symbol, ParsedVariable::Bitfield8 { value: 0, size: 0 });
                        }
                        let variable_value = result.try_get_u32(*symbol)? | (value << index);
                        let bitfield_size = result.try_get_size(*symbol)?;
                        let current_span = index + size;
                        ParsedVariable::Bitfield32 {
                            value: variable_value,
                            size: max(bitfield_size, current_span),
                        }
                    } else {
                        if *size > 8 {
                            ParsedVariable::Bitfield32 { value, size: *size }
                        } else {
                            ParsedVariable::Bitfield8 {
                                value: value as u8,
                                size: *size,
                            }
                        }
                    }
                };
                result.add(*symbol, parsed_variable);
            }
            ParsingInstruction::ConditionalOperation => {
                let z = read_bool(&mut temp_opcode);
                let creg = read_u32(&mut temp_opcode, 3) as u8;
                result.add(
                    FormatSymbol::ConditionalOperation,
                    ParsedVariable::ConditionalOperation(ConditionalOperation::from(creg, z)),
                );
            }
        }
    }

    Ok(result)
}

#[derive(Default)]
pub struct ParsingResult {
    variables: HashMap<FormatSymbol, ParsedVariable>,
}

impl ParsingResult {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, symbol: FormatSymbol, variable: ParsedVariable) {
        self.variables.insert(symbol, variable);
    }

    pub fn try_get_raw(&self, symbol: FormatSymbol) -> Result<&ParsedVariable> {
        let Some(value) = self.variables.get(&symbol) else {
            return Err(Error::other("Parsing error"));
        };
        Ok(value)
    }

    pub fn try_get_bool(&self, symbol: FormatSymbol) -> Result<bool> {
        let variable = self.try_get_raw(symbol)?;
        variable.get_bool()
    }

    pub fn try_get_bool_vec(&self, symbol: FormatSymbol) -> Result<Vec<bool>> {
        let variable = self.try_get_raw(symbol)?;
        variable.get_bool_vec()
    }

    pub fn try_get_u32(&self, symbol: FormatSymbol) -> Result<u32> {
        let variable = self.try_get_raw(symbol)?;
        variable.get_u32()
    }

    pub fn try_get_i32(&self, symbol: FormatSymbol) -> Result<i32> {
        let variable = self.try_get_raw(symbol)?;
        variable.get_i32()
    }

    pub fn try_get_u8(&self, symbol: FormatSymbol) -> Result<u8> {
        let variable = self.try_get_raw(symbol)?;
        variable.get_u8()
    }

    pub fn try_get_size(&self, symbol: FormatSymbol) -> Result<u8> {
        let variable = self.try_get_raw(symbol)?;
        variable.get_size()
    }

    pub fn try_get_lsd_unit(&self) -> Result<Unit> {
        let value = self.try_get_u8(FormatSymbol::Unit)?;
        match value {
            0b00 => Ok(Unit::L),
            0b01 => Ok(Unit::S),
            0b10 => Ok(Unit::D),
            _ => Err(Error::other("LSD unit value undefined")),
        }
    }

    pub fn try_get_register(&self, symbol: FormatSymbol) -> Result<Register> {
        let variable = self.try_get_raw(symbol)?;
        let value = variable.get_u8()?;
        Ok(Register::from(value, false))
    }

    pub fn try_get_conditional_operation(&self) -> Result<Option<ConditionalOperation>> {
        let variable = self.try_get_raw(FormatSymbol::ConditionalOperation)?;
        variable.get_conditional_operation()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingInstruction {
    Match {
        size: u8,
        value: u32,
    },
    Bit {
        symbol: FormatSymbol,
    },
    BitMatch {
        symbol: FormatSymbol,
        value: bool,
    },
    BitArray {
        size: u8,
        symbol: FormatSymbol,
    },
    Bitfield {
        size: u8,
        symbol: FormatSymbol,
    },
    BitfieldChunk {
        size: u8,
        symbol: FormatSymbol,
        index: u8,
    },
    ConditionalOperation,
}

#[derive(Clone)]
pub enum ParsedVariable {
    Bool(bool),
    BoolVec(Vec<bool>),
    Bitfield32 { value: u32, size: u8 },
    Bitfield8 { value: u8, size: u8 },
    ConditionalOperation(Option<ConditionalOperation>),
}

impl ParsedVariable {
    pub fn get_bool(&self) -> Result<bool> {
        if let ParsedVariable::Bool(value) = self {
            Ok(*value)
        } else {
            Err(Error::other("Not a Bool variable"))
        }
    }

    pub fn get_bool_vec(&self) -> Result<Vec<bool>> {
        if let ParsedVariable::BoolVec(value) = self {
            Ok(value.clone())
        } else {
            Err(Error::other("Not a BoolVec variable"))
        }
    }

    pub fn get_u32(&self) -> Result<u32> {
        if let ParsedVariable::Bitfield32 { value, .. } = self {
            Ok(*value)
        } else if let ParsedVariable::Bitfield8 { value, .. } = self {
            Ok(*value as u32)
        } else {
            Err(Error::other("Not a bitfield"))
        }
    }

    pub fn get_size(&self) -> Result<u8> {
        match self {
            ParsedVariable::Bitfield32 { size, .. } | ParsedVariable::Bitfield8 { size, .. } => {
                Ok(*size)
            }
            _ => Err(Error::other("Not a bitfield")),
        }
    }

    pub fn get_i32(&self) -> Result<i32> {
        if let ParsedVariable::Bitfield32 { value, size } = self {
            Ok(i32_from_bitfield(*value, *size))
        } else if let ParsedVariable::Bitfield8 { value, size } = self {
            Ok(i32_from_bitfield(*value as u32, *size))
        } else {
            Err(Error::other("Not a bitfield"))
        }
    }

    pub fn get_u8(&self) -> Result<u8> {
        if let ParsedVariable::Bitfield8 { value, .. } = self {
            Ok(*value)
        } else if let ParsedVariable::Bitfield32 { value, .. } = self {
            Ok(*value as u8)
        } else {
            Err(Error::other("Not a bitfield"))
        }
    }

    pub fn get_conditional_operation(&self) -> Result<Option<ConditionalOperation>> {
        if let ParsedVariable::ConditionalOperation(value) = self {
            Ok(*value)
        } else {
            Err(Error::other("Not a Conditional Operation variable"))
        }
    }
}

fn read_bool(opcode: &mut u32) -> bool {
    let value = if *opcode & 1 == 1 { true } else { false };
    *opcode >>= 1;
    value
}

fn i32_from_bitfield(bitfield: u32, size: u8) -> i32 {
    let mask = create_mask(size);
    let mut value_u32 = bitfield & mask;
    let sign_bit_mask = 1 << (size - 1);
    let value = {
        if value_u32 & sign_bit_mask == sign_bit_mask {
            value_u32 ^= mask;
            value_u32 += 1;
            -(value_u32 as i32)
        } else {
            value_u32 as i32
        }
    };
    value
}

fn read_u32(opcode: &mut u32, size: u8) -> u32 {
    let mask = create_mask(size);
    let value = *opcode & mask;
    *opcode >>= size;
    value
}

fn create_mask(size: u8) -> u32 {
    let mut mask = 0u32;
    for _ in 0..size {
        mask <<= 1;
        mask += 1;
    }
    mask
}
