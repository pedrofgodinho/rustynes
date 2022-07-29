use std::fmt::{Display, Formatter};
use crate::cpu::AddressingMode;

pub struct Instruction {
    pub opcode: u8,
    pub operands: Vec<u8>,
    pub instruction: &'static str,
    pub addressing_mode: AddressingMode,
    pub length: u16,
}


impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            opcode: 0x00,
            operands: vec![],
            instruction: "??",
            addressing_mode: AddressingMode::Implied,
            length: 1,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.addressing_mode {
            AddressingMode::Immediate => {
                write!(f, "{} #{:02X}", self.instruction, self.operands[0])
            }
            AddressingMode::ZeroPage => {
                write!(f, "{} ${:02X}", self.instruction, self.operands[0])
            }
            AddressingMode::ZeroPageX => {
                write!(f, "{} ${:02X},X", self.instruction, self.operands[0])
            }
            AddressingMode::ZeroPageY => {
                write!(f, "{} ${:02X},Y", self.instruction, self.operands[0])
            }
            AddressingMode::Absolute => {
                write!(f, "{} ${:02X}{:02X}", self.instruction, self.operands[1], self.operands[0])
            }
            AddressingMode::AbsoluteX => {
                write!(f, "{} ${:02X}{:02X},X", self.instruction, self.operands[1], self.operands[0])
            }
            AddressingMode::AbsoluteY => {
                write!(f, "{} ${:02X}{:02X},Y", self.instruction, self.operands[1], self.operands[0])
            }
            AddressingMode::Indirect => {
                write!(f, "{} (${:02X}{:02X})", self.instruction, self.operands[1], self.operands[0])
            }
            AddressingMode::IndirectX => {
                write!(f, "{} (${:02X},X)", self.instruction, self.operands[0])
            }
            AddressingMode::IndirectY => {
                write!(f, "{} (${:02X}),Y", self.instruction, self.operands[0])
            }
            AddressingMode::Relative => {
                write!(f, "{} ${:02X}", self.instruction, self.operands[0])
            }
            AddressingMode::Accumulator => {
                write!(f, "{} A", self.instruction)
            }
            AddressingMode::Implied => {
                write!(f, "{}", self.instruction)
            }
        }
    }
}