mod operations;
#[cfg(test)]
mod test;

use crate::memory::Bus;
use crate::EmulationError;

const STACK_BASE: u16 = 0x0100;
const STACK_RESET: u8 = 0xff;

#[derive(Debug)]
enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Accumulator,
    Implied,
}

#[derive(Copy, Clone)]
pub struct CpuStatus {
    pub status: u8,
}

impl CpuStatus {
    fn new() -> CpuStatus {
        CpuStatus { status: 0x00 }
    }

    fn reset(&mut self) {
        self.status = 0x00;
    }

    pub fn get_negative(&self) -> bool {
        self.status & 0b1000_0000 != 0
    }

    pub fn set_negative(&mut self, value: bool) {
        if value {
            self.status |= 0b1000_0000;
        } else {
            self.status &= !0b1000_0000;
        }
    }

    pub fn get_overflow(&self) -> bool {
        self.status & 0b0100_0000 != 0
    }

    pub fn set_overflow(&mut self, value: bool) {
        if value {
            self.status |= 0b0100_0000;
        } else {
            self.status &= !0b0100_0000;
        }
    }

    pub fn get_break(&self) -> bool {
        self.status & 0b0001_0000 != 0
    }

    pub fn set_break(&mut self, value: bool) {
        if value {
            self.status |= 0b0001_0000;
        } else {
            self.status &= !0b0001_0000;
        }
    }

    pub fn get_break_2(&self) -> bool {
        self.status & 0b0010_0000 != 0
    }

    pub fn set_break_2(&mut self, value: bool) {
        if value {
            self.status |= 0b0010_0000;
        } else {
            self.status &= !0b0010_0000;
        }
    }

    pub fn get_decimal(&self) -> bool {
        self.status & 0b0000_1000 != 0
    }

    pub fn set_decimal(&mut self, value: bool) {
        if value {
            self.status |= 0b0000_1000;
        } else {
            self.status &= !0b0000_1000;
        }
    }

    pub fn get_interrupt(&self) -> bool {
        self.status & 0b0000_0100 != 0
    }

    pub fn set_interrupt(&mut self, value: bool) {
        if value {
            self.status |= 0b0000_0100;
        } else {
            self.status &= !0b0000_0100;
        }
    }

    pub fn get_zero(&self) -> bool {
        self.status & 0b0000_0010 != 0
    }

    pub fn set_zero(&mut self, value: bool) {
        if value {
            self.status |= 0b0000_0010;
        } else {
            self.status &= !0b0000_0010;
        }
    }

    pub fn get_carry(&self) -> bool {
        self.status & 0b0000_0001 != 0
    }

    pub fn set_carry(&mut self, value: bool) {
        if value {
            self.status |= 0b0000_0001;
        } else {
            self.status &= !0b0000_0001;
        }
    }

    pub fn update_zero(&mut self, value: u8) {
        if value == 0 {
            self.set_zero(true);
        } else {
            self.set_zero(false);
        }
    }

    pub fn update_negative(&mut self, value: u8) {
        if value & 0b1000_0000 != 0 {
            self.set_negative(true);
        } else {
            self.set_negative(false);
        }
    }
}

pub struct Cpu {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub register_sp: u8,
    pub register_pc: u16,
    pub status_flags: CpuStatus,
    pub bus: Box<dyn Bus + Send + Sync>,
    pub halted: bool,
}

impl Cpu {
    pub fn new(bus: Box<dyn Bus + Send + Sync>) -> Cpu {
        Cpu {
            register_a: 0x00,
            register_x: 0x00,
            register_y: 0x00,
            register_sp: STACK_RESET,
            register_pc: 0x0000,
            status_flags: CpuStatus::new(),
            bus,
            halted: false,
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0x00;
        self.register_x = 0x00;
        self.register_y = 0x00;
        self.register_sp = STACK_RESET;
        self.register_pc = self.bus.read_word(0xFFFC).unwrap();
        self.status_flags.reset();
        self.halted = false;
        self.bus.reset();
    }

    pub fn run(&mut self) -> Result<(), EmulationError> {
        while !self.halted {
            let opcode = self.bus.read(self.register_pc)?;
            self.register_pc = self.register_pc.wrapping_add(1);

            self.handle_opcode(opcode)?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<bool, EmulationError> {
        if !self.halted {
            let opcode = self.bus.read(self.register_pc)?;
            self.register_pc = self.register_pc.wrapping_add(1);

            self.handle_opcode(opcode)?;
        }
        Ok(!self.halted)
    }

    fn halt(&mut self) {
        self.halted = true;
    }
}
