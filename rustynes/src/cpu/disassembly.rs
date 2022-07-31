use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, CpuStatus};

pub struct Instruction {
    pub opcode: u8,
    pub operands: Vec<u8>,
    pub address: u16,
    pub instruction: &'static str,
    pub addressing_mode: AddressingMode,
    pub length: u16,
}

pub struct Trace {
    pub address: u16,
    pub instruction: Instruction,
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub register_sp: u8,
    pub register_pc: u16,
    pub data_at_x: u16,
    pub data_at_y: u16,
    pub data_address: u16,
    pub data_at_address: u16,
    pub status_flags: CpuStatus,
}


impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            opcode: 0x00,
            operands: vec![],
            address: 0x00,
            instruction: "??",
            addressing_mode: AddressingMode::Implied,
            length: 1,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        f.pad(&match self.addressing_mode {
            AddressingMode::Immediate => {
                format!("{} #${:02X}", self.instruction.to_uppercase(), self.operands[0])
            }
            AddressingMode::ZeroPage => {
                format!("{} ${:02X}", self.instruction.to_uppercase(), self.operands[0])
            }
            AddressingMode::ZeroPageX => {
                format!("{} ${:02X},X", self.instruction.to_uppercase(), self.operands[0])
            }
            AddressingMode::ZeroPageY => {
                format!("{} ${:02X},Y", self.instruction.to_uppercase(), self.operands[0])
            }
            AddressingMode::Absolute => {
                format!("{} ${:02X}{:02X}", self.instruction.to_uppercase(), self.operands[1], self.operands[0])
            }
            AddressingMode::AbsoluteX => {
                format!("{} ${:02X}{:02X},X", self.instruction.to_uppercase(), self.operands[1], self.operands[0])
            }
            AddressingMode::AbsoluteY => {
                format!("{} ${:02X}{:02X},Y", self.instruction.to_uppercase(), self.operands[1], self.operands[0])
            }
            AddressingMode::Indirect => {
                format!("{} (${:02X}{:02X})", self.instruction.to_uppercase(), self.operands[1], self.operands[0])
            }
            AddressingMode::IndirectX => {
                format!("{} (${:02X},X)", self.instruction.to_uppercase(), self.operands[0])
            }
            AddressingMode::IndirectY => {
                format!("{} (${:02X}),Y", self.instruction.to_uppercase(), self.operands[0])
            }
            AddressingMode::Relative => {
                format!("{} ${:02X}", self.instruction.to_uppercase(), self.address.wrapping_add(self.operands[0] as u16).wrapping_add(2))
            }
            AddressingMode::Accumulator => {
                format!("{} A", self.instruction.to_uppercase())
            }
            AddressingMode::Implied => {
                self.instruction.to_string().to_uppercase()
            }
        })
    }
}


impl Display for Trace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}  ", self.address)?;
        write!(f, "{:02X} ", self.instruction.opcode)?;
        if self.instruction.length > 1 {
            write!(f, "{:02X} ", self.instruction.operands[0])?;
        } else {
            write!(f, "   ")?;
        }
        if self.instruction.length > 2 {
            write!(f, "{:02X}  ", self.instruction.operands[1])?;
        } else {
            write!(f, "    ")?;
        }

        let instruction = match self.instruction.addressing_mode {
            AddressingMode::ZeroPage => format!("{} = {:02X}", self.instruction, self.data_at_address as u8),
            AddressingMode::Absolute => {
                if self.instruction.instruction != "jmp" && self.instruction.instruction != "jsr" {
                    format!("{} = {:02X}", self.instruction, self.data_at_address as u8)
                } else {
                    format!("{}", self.instruction)
                }
            },
            AddressingMode::ZeroPageX | AddressingMode::ZeroPageY =>
                format!("{} @ {:02X} = {:02X}", self.instruction, self.data_address, self.data_at_address as u8),
            AddressingMode::AbsoluteX | AddressingMode::AbsoluteY =>
                format!("{} @ {:04X} = {:02X}", self.instruction, self.data_address, self.data_at_address as u8),
            AddressingMode::Indirect => format!("{} = {:04X}", self.instruction, self.data_address),
            AddressingMode::IndirectX => format!("{} @ {:02X} = {:04X} = {:02X}", self.instruction, self.register_x.wrapping_add(self.instruction.operands[0]), self.data_address, self.data_at_address as u8),
            AddressingMode::IndirectY => format!("{} = {:04X} @ {:04X} = {:02X}", self.instruction, self.data_address.wrapping_sub(self.register_y as u16), self.data_address, self.data_at_address as u8),
            _ => self.instruction.to_string(),
        };
        write!(f, "{:<32}", instruction)?;
        write!(f, "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}", self.register_a, self.register_x, self.register_y, self.status_flags.status, self.register_sp)

    }
}