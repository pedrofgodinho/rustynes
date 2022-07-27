use crate::cpu::{AddressingMode, Cpu, STACK_BASE};
use crate::EmulationError;


struct OpResult {
    extra_cycles: u8,
    increment_pc: bool,
}

impl OpResult {
    fn new(extra_cycles: u8, increment_pc: bool) -> OpResult {
        OpResult {
            extra_cycles,
            increment_pc,
        }
    }
}


impl Cpu {
    pub(super) fn handle_opcode(&mut self, opcode: u8) -> Result<u8, EmulationError> {
        macro_rules! match_op {
            ($to_match:expr; $($instruction:ident: $($opcode:expr => $mode:ident ($bytes:expr)),+;)+) => {
                return match $to_match {
                    $($($opcode => {
                        let op_result = self.$instruction(AddressingMode::$mode)?;
                        if op_result.increment_pc {
                            self.register_pc = self.register_pc.wrapping_add($bytes - 1);
                        }
                        Ok(op_result.extra_cycles)
                    },)+)+
                    _ => Err(EmulationError::InvalidOpcode(opcode)),
                }
            };
        }
        include!("operations_include.rs");
    }

    pub fn disassemble(&mut self, pc: u16) -> Result<(String, u16), EmulationError> {
        let opcode = self.memory.read(pc)?;
        macro_rules! match_op {
            ($to_match:expr; $($instruction:ident: $($opcode:expr => $mode:ident ($bytes:expr)),+;)+) => {
                return match $to_match {
                    $($($opcode => {
                        let instruction_name = stringify!($instruction);
                        match $bytes {
                            1 => Ok((format!("{}", instruction_name), $bytes)),
                            2 => Ok((format!("{} {:?} ${:02x}", instruction_name, AddressingMode::$mode, self.memory.read(pc+1).unwrap()), $bytes)),
                            3 => Ok((format!("{} {:?} ${:04x}", instruction_name, AddressingMode::$mode, self.memory.read_word(pc+1).unwrap()), $bytes)),
                            _ => Err(EmulationError::InvalidOpcode(opcode)),
                        }
                    },)+)+
                    _ => Err(EmulationError::InvalidOpcode(opcode)),
                }
            };
        }
        include!("operations_include.rs");
    }


    fn get_operand_address(&self, mode: AddressingMode) -> Result<u16, EmulationError> {
        // TODO Test properly
        match mode {
            AddressingMode::Immediate => Ok(self.register_pc),
            AddressingMode::ZeroPage => Ok(self.memory.read(self.register_pc)? as u16),
            AddressingMode::ZeroPageX => Ok(self.memory.read(self.register_pc)?.wrapping_add(self.register_x) as u16),
            AddressingMode::ZeroPageY => Ok(self.memory.read(self.register_pc)?.wrapping_add(self.register_y) as u16),
            AddressingMode::Absolute => Ok(self.memory.read_word(self.register_pc)?),
            AddressingMode::AbsoluteX => Ok(self.memory.read_word(self.register_pc)?.wrapping_add(self.register_x as u16)),
            AddressingMode::AbsoluteY => Ok(self.memory.read_word(self.register_pc)?.wrapping_add(self.register_y as u16)),
            AddressingMode::Indirect => {
                // Emulate the 6502 bug of wrapping around the address space when the low byte of the address is 0xFF.
                let address = self.memory.read_word(self.register_pc)?;
                if address & 0x00FF == 0x00FF {
                    Ok(u16::from_le_bytes([self.memory.read(address)?, self.memory.read(address & 0xFF00)?]))
                } else {
                    Ok(self.memory.read_word(address)?)
                }

            },
            AddressingMode::Relative => Ok(self.register_pc),
            AddressingMode::IndirectX => {
                let base = self.memory.read(self.register_pc)?;
                let ptr = base.wrapping_add(self.register_x);
                let lo = self.memory.read(ptr as u16)?;
                let hi = self.memory.read(ptr.wrapping_add(1) as u16)?;
                Ok(u16::from_le_bytes([lo, hi]))
            },
            AddressingMode::IndirectY => {
                let base = self.memory.read(self.register_pc)?;
                let lo = self.memory.read(base as u16)?;
                let hi = self.memory.read(base.wrapping_add(1) as u16)?;
                let deref_base = u16::from_le_bytes([lo, hi]);
                Ok(deref_base.wrapping_add(self.register_y as u16))
            },
            _ => panic!("Unsupported addressing mode: {:?}", mode),
        }
    }


    fn stack_push(&mut self, value: u8) -> Result<(), EmulationError> {
        self.memory.write(STACK_BASE + self.register_sp as u16, value)?;
        self.register_sp = self.register_sp.wrapping_sub(1);
        Ok(())
    }

    fn stack_pop(&mut self) -> Result<u8, EmulationError> {
        self.register_sp = self.register_sp.wrapping_add(1);
        self.memory.read(STACK_BASE + self.register_sp as u16)
    }

    fn stack_push_word(&mut self, value: u16) -> Result<(), EmulationError> {
        self.stack_push((value >> 8) as u8)?;
        self.stack_push((value & 0xff) as u8)?;
        Ok(())
    }

    fn stack_pop_word(&mut self) -> Result<u16, EmulationError> {
        let lo = self.stack_pop()?;
        let hi = self.stack_pop()?;
        Ok(u16::from_le_bytes([lo, hi]))
    }

    fn add_to_register_a(&mut self, data: u8) {
        let sum = self.register_a as u16 + data as u16 + self.status_flags.get_carry() as u16;

        let carry= sum > 0xFF;

        self.status_flags.set_carry(carry);
        let result = sum as u8;
        self.status_flags.set_overflow((data ^ result) & (result ^ self.register_a) & 0x80 != 0);

        self.register_a = result;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
    }

    fn branch_aux(&mut self, mode: AddressingMode, condition: bool) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let jump = self.memory.read(addr)? as i8;
        if condition {
            self.register_pc = addr.wrapping_add(1).wrapping_add(jump as u16);
            Ok(OpResult::new(0, false))
        } else {
            Ok(OpResult::new(0, true))
        }
    }

    fn compare_aux(&mut self, mode: AddressingMode, compare_with: u8) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let value = self.memory.read(addr)?;
        self.status_flags.set_carry(value < compare_with);
        let result = compare_with.wrapping_sub(value);
        self.status_flags.update_zero(result);
        self.status_flags.update_negative(result);
        Ok(OpResult::new(0, true))
    }



    // Ignoring the decimal mode since it is not used in the NES.
    fn adc(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let value = self.memory.read(addr)?;
        self.add_to_register_a(value);
        Ok(OpResult::new(0, true))
    }

    fn and(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.register_a &= self.memory.read(addr)?;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
        Ok(OpResult::new(0, true))
    }

    fn asl(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let old;
        let new;
        match mode {
            AddressingMode::Accumulator => {
                old = self.register_a;
                self.register_a <<= 1;
                new = self.register_a;
            },
            _ => {
                let addr = self.get_operand_address(mode)?;
                old = self.memory.read(addr)?;
                self.memory.write(addr, old << 1)?;
                new = old << 1;
            }
        }
        self.status_flags.set_carry(old & 0x08 != 0);
        self.status_flags.update_negative(new);
        self.status_flags.update_zero(new);
        Ok(OpResult::new(0, true))
    }

    fn bcc(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.branch_aux(mode, !self.status_flags.get_carry())
    }

    fn bcs(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.branch_aux(mode, self.status_flags.get_carry())
    }

    fn beq(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.branch_aux(mode, self.status_flags.get_zero())
    }

    fn bit(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let value = self.memory.read(addr)?;
        self.status_flags.set_overflow(value & 0x40 != 0);
        self.status_flags.update_negative(value);
        self.status_flags.update_zero(self.register_a & value);
        Ok(OpResult::new(0, true))
    }

    fn bmi(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.branch_aux(mode, self.status_flags.get_negative())
    }

    fn bne(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.branch_aux(mode, !self.status_flags.get_zero())
    }

    fn bpl(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.branch_aux(mode, !self.status_flags.get_negative())
    }

    fn brk(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.halt();
        Ok(OpResult::new(0, true))
    }

    fn bvc(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.branch_aux(mode, !self.status_flags.get_overflow())
    }

    fn bvs(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.branch_aux(mode, self.status_flags.get_overflow())
    }

    fn clc(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.status_flags.set_carry(false);
        Ok(OpResult::new(0, true))
    }

    fn cld(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.status_flags.set_decimal(false);
        Ok(OpResult::new(0, true))
    }

    fn cli(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.status_flags.set_interrupt(false);
        Ok(OpResult::new(0, true))
    }

    fn clv(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.status_flags.set_overflow(false);
        Ok(OpResult::new(0, true))
    }



    fn cmp(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.compare_aux(mode, self.register_a)
    }

    fn cpx(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.compare_aux(mode, self.register_x)
    }

    fn cpy(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.compare_aux(mode, self.register_y)
    }

    fn dec(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let mut value = self.memory.read(addr)?;
        value = value.wrapping_sub(1);
        self.memory.write(addr, value)?;
        self.status_flags.update_negative(value);
        self.status_flags.update_zero(value);
        Ok(OpResult::new(0, true))
    }

    fn dex(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_x = self.register_x.wrapping_sub(1);
        self.status_flags.update_negative(self.register_x);
        self.status_flags.update_zero(self.register_x);
        Ok(OpResult::new(0, true))
    }

    fn dey(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_y = self.register_y.wrapping_sub(1);
        self.status_flags.update_negative(self.register_y);
        self.status_flags.update_zero(self.register_y);
        Ok(OpResult::new(0, true))
    }

    fn eor(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let value = self.memory.read(addr)?;
        self.register_a ^= value;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
        Ok(OpResult::new(0, true))
    }

    fn inc(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let mut value = self.memory.read(addr)?;
        value = value.wrapping_add(1);
        self.memory.write(addr, value)?;
        self.status_flags.update_negative(value);
        self.status_flags.update_zero(value);
        Ok(OpResult::new(0, true))
    }

    fn inx(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_x = self.register_x.wrapping_add(1);
        self.status_flags.update_negative(self.register_x);
        self.status_flags.update_zero(self.register_x);
        Ok(OpResult::new(0, true))
    }

    fn iny(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_y = self.register_y.wrapping_add(1);
        self.status_flags.update_negative(self.register_y);
        self.status_flags.update_zero(self.register_y);
        Ok(OpResult::new(0, true))
    }

    fn jmp(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.register_pc = addr;
        Ok(OpResult::new(0, false))
    }

    fn jsr(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.stack_push_word(self.register_pc.wrapping_add(2))?;
        self.register_pc = addr;
        Ok(OpResult::new(0, false))
    }

    fn lda(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.register_a = self.memory.read(addr)?;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
        Ok(OpResult::new(0, true))
    }

    fn ldx(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.register_x = self.memory.read(addr)?;
        self.status_flags.update_negative(self.register_x);
        self.status_flags.update_zero(self.register_x);
        Ok(OpResult::new(0, true))
    }

    fn ldy(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.register_y = self.memory.read(addr)?;
        self.status_flags.update_negative(self.register_y);
        self.status_flags.update_zero(self.register_y);
        Ok(OpResult::new(0, true))
    }

    fn lsr(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let old;
        let new;
        match mode {
            AddressingMode::Accumulator => {
                old = self.register_a;
                self.register_a >>= 1;
                new = self.register_a;
            },
            _ => {
                let addr = self.get_operand_address(mode)?;
                old = self.memory.read(addr)?;
                self.memory.write(addr, old >> 1)?;
                new = old >> 1;
            }
        }
        self.status_flags.set_carry(old & 0x01 != 0);
        self.status_flags.update_negative(new);
        self.status_flags.update_zero(new);
        Ok(OpResult::new(0, true))
    }

    fn nop(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        Ok(OpResult::new(0, true))
    }

    fn ora(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let value = self.memory.read(addr)?;
        self.register_a |= value;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
        Ok(OpResult::new(0, true))
    }

    fn pha(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.stack_push(self.register_a)?;
        Ok(OpResult::new(0, true))
    }

    fn php(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let mut state = self.status_flags; // clone
        state.set_break(true);
        state.set_break_2(true);
        self.stack_push(state.status)?;
        Ok(OpResult::new(0, true))
    }

    fn pla(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_a = self.stack_pop()?;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
        Ok(OpResult::new(0, true))
    }

    fn plp(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.status_flags.status = self.stack_pop()?;
        self.status_flags.set_break(false);
        self.status_flags.set_break_2(true);
        Ok(OpResult::new(0, true))
    }

    fn rol(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let old;
        let new;
        match mode {
            AddressingMode::Accumulator => {
                old = self.register_a;
                self.register_a = self.register_a.rotate_left(1);
                new = self.register_a;
            },
            _ => {
                let addr = self.get_operand_address(mode)?;
                old = self.memory.read(addr)?;
                self.memory.write(addr, old.rotate_left(1))?;
                new = old.rotate_left(1);
            }
        }
        self.status_flags.set_carry(old & 0x80 != 0);
        self.status_flags.update_negative(new);
        self.status_flags.update_zero(new);
        Ok(OpResult::new(0, true))
    }

    fn ror(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let old;
        let new;
        match mode {
            AddressingMode::Accumulator => {
                old = self.register_a;
                self.register_a = self.register_a.rotate_right(1);
                new = self.register_a;
            },
            _ => {
                let addr = self.get_operand_address(mode)?;
                old = self.memory.read(addr)?;
                self.memory.write(addr, old.rotate_right(1))?;
                new = old.rotate_right(1);
            }
        }
        self.status_flags.set_carry(old & 0x01 != 0);
        self.status_flags.update_negative(new);
        self.status_flags.update_zero(new);
        Ok(OpResult::new(0, true))
    }

    fn rti(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.status_flags.status = self.stack_pop()?;
        self.status_flags.set_break(false);
        self.status_flags.set_break_2(true);
        self.register_pc = self.stack_pop_word()?;
        Ok(OpResult::new(0, true))
    }

    fn rts(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_pc = self.stack_pop_word()?;
        Ok(OpResult::new(0, false))
    }

    fn sbc(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let value = self.memory.read(addr)?;
        self.add_to_register_a((value as i8).wrapping_neg().wrapping_sub(1) as u8);
        Ok(OpResult::new(0, true))
    }

    fn sec(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.status_flags.set_carry(true);
        Ok(OpResult::new(0, true))
    }

    fn sed(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.status_flags.set_decimal(true);
        Ok(OpResult::new(0, true))
    }

    fn sei(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.status_flags.set_interrupt(true);
        Ok(OpResult::new(0, true))
    }

    fn sta(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.memory.write(addr, self.register_a)?;
        Ok(OpResult::new(0, true))
    }

    fn stx(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.memory.write(addr, self.register_x)?;
        Ok(OpResult::new(0, true))
    }

    fn sty(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.memory.write(addr, self.register_y)?;
        Ok(OpResult::new(0, true))
    }

    fn tax(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_x = self.register_a;
        self.status_flags.update_negative(self.register_x);
        self.status_flags.update_zero(self.register_x);
        Ok(OpResult::new(0, true))
    }

    fn tay(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_y = self.register_a;
        self.status_flags.update_negative(self.register_y);
        self.status_flags.update_zero(self.register_y);
        Ok(OpResult::new(0, true))
    }

    fn tsx(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_x = self.register_sp;
        self.status_flags.update_negative(self.register_x);
        self.status_flags.update_zero(self.register_x);
        Ok(OpResult::new(0, true))
    }

    fn txa(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_a = self.register_x;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
        Ok(OpResult::new(0, true))
    }

    fn txs(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_sp = self.register_x;
        Ok(OpResult::new(0, true))
    }

    fn tya(&mut self, _mode: AddressingMode) -> Result<OpResult, EmulationError> {
        self.register_a = self.register_y;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
        Ok(OpResult::new(0, true))
    }
}