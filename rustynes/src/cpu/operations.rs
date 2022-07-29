use emulator_macros::{disassemble_op, instruction_match};
use crate::cpu::{AddressingMode, Cpu, STACK_BASE};
use crate::cpu::disassembly::Instruction;
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
        instruction_match!(
            opcode {
                adc: 0x69 => Immediate (2), 0x65 => ZeroPage (2), 0x75 => ZeroPageX (2), 0x6D => Absolute (3), 0x7D => AbsoluteX (3), 0x79 => AbsoluteY (3), 0x61 => IndirectX (2), 0x71 => IndirectY (2);
                and: 0x29 => Immediate (2), 0x25 => ZeroPage (2), 0x35 => ZeroPageX (2), 0x2D => Absolute (3), 0x3D => AbsoluteX (3), 0x39 => AbsoluteY (3), 0x21 => IndirectX (2), 0x31 => IndirectY (2);
                asl: 0x0A => Accumulator (1), 0x06 => ZeroPage (2), 0x16 => ZeroPageX (2), 0x0E => Absolute (3), 0x1E => AbsoluteX (3);
                bcc: 0x90 => Relative (2);
                bcs: 0xB0 => Relative (2);
                beq: 0xF0 => Relative (2);
                bit: 0x24 => ZeroPage (2), 0x2C => Absolute (3);
                bmi: 0x30 => Relative (2);
                bne: 0xD0 => Relative (2);
                bpl: 0x10 => Relative (2);
                brk: 0x00 => Implied (1);
                bvc: 0x50 => Relative (2);
                bvs: 0x70 => Relative (2);
                clc: 0x18 => Implied (1);
                cld: 0xD8 => Implied (1);
                cli: 0x58 => Implied (1);
                clv: 0xB8 => Implied (1);
                cmp: 0xC9 => Immediate (2), 0xC5 => ZeroPage (2), 0xD5 => ZeroPageX (2), 0xCD => Absolute (3), 0xDD => AbsoluteX (3), 0xD9 => AbsoluteY (3), 0xC1 => IndirectX (2), 0xD1 => IndirectY (2);
                cpx: 0xE0 => Immediate (2), 0xE4 => ZeroPage (2), 0xEC => Absolute (3);
                cpy: 0xC0 => Immediate (2), 0xC4 => ZeroPage (2), 0xCC => Absolute (3);
                dec: 0xC6 => ZeroPage (2), 0xD6 => ZeroPageX (2), 0xCE => Absolute (3), 0xDE => AbsoluteX (3);
                dex: 0xCA => Implied (1);
                dey: 0x88 => Implied (1);
                eor: 0x49 => Immediate (2), 0x45 => ZeroPage (2), 0x55 => ZeroPageX (2), 0x4D => Absolute (3), 0x5D => AbsoluteX (3), 0x59 => AbsoluteY (3), 0x41 => IndirectX (2), 0x51 => IndirectY (2);
                inc: 0xE6 => ZeroPage (2), 0xF6 => ZeroPageX (2), 0xEE => Absolute (3), 0xFE => AbsoluteX (3);
                inx: 0xE8 => Implied (1);
                iny: 0xC8 => Implied (1);
                jmp: 0x4C => Absolute (3), 0x6C => Indirect  (3);
                jsr: 0x20 => Absolute (3);
                lda: 0xA9 => Immediate (2), 0xA5 => ZeroPage (2), 0xB5 => ZeroPageX (2), 0xAD => Absolute (3), 0xBD => AbsoluteX (3), 0xB9 => AbsoluteY (3), 0xA1 => IndirectX (2), 0xB1 => IndirectY (2);
                ldx: 0xA2 => Immediate (2), 0xA6 => ZeroPage (2), 0xB6 => ZeroPageY (2), 0xAE => Absolute (3), 0xBE => AbsoluteY (3);
                ldy: 0xA0 => Immediate (2), 0xA4 => ZeroPage (2), 0xB4 => ZeroPageX (2), 0xAC => Absolute (3), 0xBC => AbsoluteX (3);
                lsr: 0x4A => Accumulator (1), 0x46 => ZeroPage (2), 0x56 => ZeroPageX (2), 0x4E => Absolute (3), 0x5E => AbsoluteX (3);
                nop: 0xEA => Implied (1);
                ora: 0x09 => Immediate (2), 0x05 => ZeroPage (2), 0x15 => ZeroPageX (2), 0x0D => Absolute (3), 0x1D => AbsoluteX (3), 0x19 => AbsoluteY (3), 0x01 => IndirectX (2), 0x11 => IndirectY (2);
                pha: 0x48 => Implied (1);
                php: 0x08 => Implied (1);
                pla: 0x68 => Implied (1);
                plp: 0x28 => Implied (1);
                rol: 0x2A => Accumulator (1), 0x26 => ZeroPage (2), 0x36 => ZeroPageX (2), 0x2E => Absolute (3), 0x3E => AbsoluteX (3);
                ror: 0x6A => Accumulator (1), 0x66 => ZeroPage (2), 0x76 => ZeroPageX (2), 0x6E => Absolute (3), 0x7E => AbsoluteX (3);
                rti: 0x40 => Implied (1);
                rts: 0x60 => Implied (1);
                sbc: 0xE9 => Immediate (2), 0xE5 => ZeroPage (2), 0xF5 => ZeroPageX (2), 0xED => Absolute (3), 0xFD => AbsoluteX (3), 0xF9 => AbsoluteY (3), 0xE1 => IndirectX (2), 0xF1 => IndirectY (2);
                sec: 0x38 => Implied (1);
                sed: 0xF8 => Implied (1);
                sei: 0x78 => Implied (1);
                sta: 0x85 => ZeroPage (2), 0x95 => ZeroPageX (2), 0x8D => Absolute (3), 0x9D => AbsoluteX (3), 0x99 => AbsoluteY (3), 0x81 => IndirectX (2), 0x91 => IndirectY (2);
                stx: 0x86 => ZeroPage (2), 0x96 => ZeroPageY (2), 0x8E => Absolute (3);
                sty: 0x84 => ZeroPage (2), 0x94 => ZeroPageX (2), 0x8C => Absolute (3);
                tax: 0xAA => Implied (1);
                tay: 0xA8 => Implied (1);
                tsx: 0xBA => Implied (1);
                txa: 0x8A => Implied (1);
                txs: 0x9A => Implied (1);
                tya: 0x98 => Implied (1);
            }
        )
    }

    pub fn disassemble(&self, pc: u16) -> Result<Instruction, EmulationError> {
        let opcode = self.bus.read(pc)?;
        disassemble_op!(
                        opcode {
                adc: 0x69 => Immediate (2), 0x65 => ZeroPage (2), 0x75 => ZeroPageX (2), 0x6D => Absolute (3), 0x7D => AbsoluteX (3), 0x79 => AbsoluteY (3), 0x61 => IndirectX (2), 0x71 => IndirectY (2);
                and: 0x29 => Immediate (2), 0x25 => ZeroPage (2), 0x35 => ZeroPageX (2), 0x2D => Absolute (3), 0x3D => AbsoluteX (3), 0x39 => AbsoluteY (3), 0x21 => IndirectX (2), 0x31 => IndirectY (2);
                asl: 0x0A => Accumulator (1), 0x06 => ZeroPage (2), 0x16 => ZeroPageX (2), 0x0E => Absolute (3), 0x1E => AbsoluteX (3);
                bcc: 0x90 => Relative (2);
                bcs: 0xB0 => Relative (2);
                beq: 0xF0 => Relative (2);
                bit: 0x24 => ZeroPage (2), 0x2C => Absolute (3);
                bmi: 0x30 => Relative (2);
                bne: 0xD0 => Relative (2);
                bpl: 0x10 => Relative (2);
                brk: 0x00 => Implied (1);
                bvc: 0x50 => Relative (2);
                bvs: 0x70 => Relative (2);
                clc: 0x18 => Implied (1);
                cld: 0xD8 => Implied (1);
                cli: 0x58 => Implied (1);
                clv: 0xB8 => Implied (1);
                cmp: 0xC9 => Immediate (2), 0xC5 => ZeroPage (2), 0xD5 => ZeroPageX (2), 0xCD => Absolute (3), 0xDD => AbsoluteX (3), 0xD9 => AbsoluteY (3), 0xC1 => IndirectX (2), 0xD1 => IndirectY (2);
                cpx: 0xE0 => Immediate (2), 0xE4 => ZeroPage (2), 0xEC => Absolute (3);
                cpy: 0xC0 => Immediate (2), 0xC4 => ZeroPage (2), 0xCC => Absolute (3);
                dec: 0xC6 => ZeroPage (2), 0xD6 => ZeroPageX (2), 0xCE => Absolute (3), 0xDE => AbsoluteX (3);
                dex: 0xCA => Implied (1);
                dey: 0x88 => Implied (1);
                eor: 0x49 => Immediate (2), 0x45 => ZeroPage (2), 0x55 => ZeroPageX (2), 0x4D => Absolute (3), 0x5D => AbsoluteX (3), 0x59 => AbsoluteY (3), 0x41 => IndirectX (2), 0x51 => IndirectY (2);
                inc: 0xE6 => ZeroPage (2), 0xF6 => ZeroPageX (2), 0xEE => Absolute (3), 0xFE => AbsoluteX (3);
                inx: 0xE8 => Implied (1);
                iny: 0xC8 => Implied (1);
                jmp: 0x4C => Absolute (3), 0x6C => Indirect  (3);
                jsr: 0x20 => Absolute (3);
                lda: 0xA9 => Immediate (2), 0xA5 => ZeroPage (2), 0xB5 => ZeroPageX (2), 0xAD => Absolute (3), 0xBD => AbsoluteX (3), 0xB9 => AbsoluteY (3), 0xA1 => IndirectX (2), 0xB1 => IndirectY (2);
                ldx: 0xA2 => Immediate (2), 0xA6 => ZeroPage (2), 0xB6 => ZeroPageY (2), 0xAE => Absolute (3), 0xBE => AbsoluteY (3);
                ldy: 0xA0 => Immediate (2), 0xA4 => ZeroPage (2), 0xB4 => ZeroPageX (2), 0xAC => Absolute (3), 0xBC => AbsoluteX (3);
                lsr: 0x4A => Accumulator (1), 0x46 => ZeroPage (2), 0x56 => ZeroPageX (2), 0x4E => Absolute (3), 0x5E => AbsoluteX (3);
                nop: 0xEA => Implied (1);
                ora: 0x09 => Immediate (2), 0x05 => ZeroPage (2), 0x15 => ZeroPageX (2), 0x0D => Absolute (3), 0x1D => AbsoluteX (3), 0x19 => AbsoluteY (3), 0x01 => IndirectX (2), 0x11 => IndirectY (2);
                pha: 0x48 => Implied (1);
                php: 0x08 => Implied (1);
                pla: 0x68 => Implied (1);
                plp: 0x28 => Implied (1);
                rol: 0x2A => Accumulator (1), 0x26 => ZeroPage (2), 0x36 => ZeroPageX (2), 0x2E => Absolute (3), 0x3E => AbsoluteX (3);
                ror: 0x6A => Accumulator (1), 0x66 => ZeroPage (2), 0x76 => ZeroPageX (2), 0x6E => Absolute (3), 0x7E => AbsoluteX (3);
                rti: 0x40 => Implied (1);
                rts: 0x60 => Implied (1);
                sbc: 0xE9 => Immediate (2), 0xE5 => ZeroPage (2), 0xF5 => ZeroPageX (2), 0xED => Absolute (3), 0xFD => AbsoluteX (3), 0xF9 => AbsoluteY (3), 0xE1 => IndirectX (2), 0xF1 => IndirectY (2);
                sec: 0x38 => Implied (1);
                sed: 0xF8 => Implied (1);
                sei: 0x78 => Implied (1);
                sta: 0x85 => ZeroPage (2), 0x95 => ZeroPageX (2), 0x8D => Absolute (3), 0x9D => AbsoluteX (3), 0x99 => AbsoluteY (3), 0x81 => IndirectX (2), 0x91 => IndirectY (2);
                stx: 0x86 => ZeroPage (2), 0x96 => ZeroPageY (2), 0x8E => Absolute (3);
                sty: 0x84 => ZeroPage (2), 0x94 => ZeroPageX (2), 0x8C => Absolute (3);
                tax: 0xAA => Implied (1);
                tay: 0xA8 => Implied (1);
                tsx: 0xBA => Implied (1);
                txa: 0x8A => Implied (1);
                txs: 0x9A => Implied (1);
                tya: 0x98 => Implied (1);
            }
        )
    }

    fn get_operand_address(&self, mode: AddressingMode) -> Result<u16, EmulationError> {
        // TODO Test properly
        match mode {
            AddressingMode::Immediate => Ok(self.register_pc),
            AddressingMode::ZeroPage => Ok(self.bus.read(self.register_pc)? as u16),
            AddressingMode::ZeroPageX => Ok(self
                .bus
                .read(self.register_pc)?
                .wrapping_add(self.register_x) as u16),
            AddressingMode::ZeroPageY => Ok(self
                .bus
                .read(self.register_pc)?
                .wrapping_add(self.register_y) as u16),
            AddressingMode::Absolute => Ok(self.bus.read_word(self.register_pc)?),
            AddressingMode::AbsoluteX => Ok(self
                .bus
                .read_word(self.register_pc)?
                .wrapping_add(self.register_x as u16)),
            AddressingMode::AbsoluteY => Ok(self
                .bus
                .read_word(self.register_pc)?
                .wrapping_add(self.register_y as u16)),
            AddressingMode::Indirect => {
                // Emulate the 6502 bug of wrapping around the address space when the low byte of the address is 0xFF.
                let address = self.bus.read_word(self.register_pc)?;
                if address & 0x00FF == 0x00FF {
                    Ok(u16::from_le_bytes([
                        self.bus.read(address)?,
                        self.bus.read(address & 0xFF00)?,
                    ]))
                } else {
                    Ok(self.bus.read_word(address)?)
                }
            }
            AddressingMode::Relative => Ok(self.register_pc),
            AddressingMode::IndirectX => {
                let base = self.bus.read(self.register_pc)?;
                let ptr = base.wrapping_add(self.register_x);
                let lo = self.bus.read(ptr as u16)?;
                let hi = self.bus.read(ptr.wrapping_add(1) as u16)?;
                Ok(u16::from_le_bytes([lo, hi]))
            }
            AddressingMode::IndirectY => {
                let base = self.bus.read(self.register_pc)?;
                let lo = self.bus.read(base as u16)?;
                let hi = self.bus.read(base.wrapping_add(1) as u16)?;
                let deref_base = u16::from_le_bytes([lo, hi]);
                Ok(deref_base.wrapping_add(self.register_y as u16))
            }
            _ => panic!("Unsupported addressing mode: {:?}", mode),
        }
    }

    fn stack_push(&mut self, value: u8) -> Result<(), EmulationError> {
        self.bus
            .write(STACK_BASE + self.register_sp as u16, value)?;
        self.register_sp = self.register_sp.wrapping_sub(1);
        Ok(())
    }

    fn stack_pop(&mut self) -> Result<u8, EmulationError> {
        self.register_sp = self.register_sp.wrapping_add(1);
        self.bus.read(STACK_BASE + self.register_sp as u16)
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

        let carry = sum > 0xFF;

        self.status_flags.set_carry(carry);
        let result = sum as u8;
        self.status_flags
            .set_overflow((data ^ result) & (result ^ self.register_a) & 0x80 != 0);

        self.register_a = result;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
    }

    fn branch_aux(
        &mut self,
        mode: AddressingMode,
        condition: bool,
    ) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let jump = self.bus.read(addr)? as i8;
        if condition {
            self.register_pc = addr.wrapping_add(1).wrapping_add(jump as u16);
            Ok(OpResult::new(0, false))
        } else {
            Ok(OpResult::new(0, true))
        }
    }

    fn compare_aux(
        &mut self,
        mode: AddressingMode,
        compare_with: u8,
    ) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let value = self.bus.read(addr)?;
        self.status_flags.set_carry(value < compare_with);
        let result = compare_with.wrapping_sub(value);
        self.status_flags.update_zero(result);
        self.status_flags.update_negative(result);
        Ok(OpResult::new(0, true))
    }

    // Ignoring the decimal mode since it is not used in the NES.
    fn adc(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let value = self.bus.read(addr)?;
        self.add_to_register_a(value);
        Ok(OpResult::new(0, true))
    }

    fn and(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.register_a &= self.bus.read(addr)?;
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
            }
            _ => {
                let addr = self.get_operand_address(mode)?;
                old = self.bus.read(addr)?;
                self.bus.write(addr, old << 1)?;
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
        let value = self.bus.read(addr)?;
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
        let mut value = self.bus.read(addr)?;
        value = value.wrapping_sub(1);
        self.bus.write(addr, value)?;
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
        let value = self.bus.read(addr)?;
        self.register_a ^= value;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
        Ok(OpResult::new(0, true))
    }

    fn inc(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        let mut value = self.bus.read(addr)?;
        value = value.wrapping_add(1);
        self.bus.write(addr, value)?;
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
        self.register_a = self.bus.read(addr)?;
        self.status_flags.update_negative(self.register_a);
        self.status_flags.update_zero(self.register_a);
        Ok(OpResult::new(0, true))
    }

    fn ldx(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.register_x = self.bus.read(addr)?;
        self.status_flags.update_negative(self.register_x);
        self.status_flags.update_zero(self.register_x);
        Ok(OpResult::new(0, true))
    }

    fn ldy(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.register_y = self.bus.read(addr)?;
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
            }
            _ => {
                let addr = self.get_operand_address(mode)?;
                old = self.bus.read(addr)?;
                self.bus.write(addr, old >> 1)?;
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
        let value = self.bus.read(addr)?;
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
            }
            _ => {
                let addr = self.get_operand_address(mode)?;
                old = self.bus.read(addr)?;
                self.bus.write(addr, old.rotate_left(1))?;
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
            }
            _ => {
                let addr = self.get_operand_address(mode)?;
                old = self.bus.read(addr)?;
                self.bus.write(addr, old.rotate_right(1))?;
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
        let value = self.bus.read(addr)?;
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
        self.bus.write(addr, self.register_a)?;
        Ok(OpResult::new(0, true))
    }

    fn stx(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.bus.write(addr, self.register_x)?;
        Ok(OpResult::new(0, true))
    }

    fn sty(&mut self, mode: AddressingMode) -> Result<OpResult, EmulationError> {
        let addr = self.get_operand_address(mode)?;
        self.bus.write(addr, self.register_y)?;
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
