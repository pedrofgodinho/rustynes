use crate::cpu::Cpu;
use crate::memory::nes::NesBus;

#[test]
fn test_lda() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xA9, 0x05, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x05);
    assert!(!cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
}

#[test]
fn test_lda_zero() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xA9, 0x00, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
    assert!(cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
}

#[test]
fn test_lda_neg() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xA9, 0xff, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xff);
    assert!(!cpu.status_flags.get_zero());
    assert!(cpu.status_flags.get_negative());
}

#[test]
fn test_tax() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xaa, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0x10;
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0x10);
    assert!(!cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
}

#[test]
fn test_5_ops_working_together() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xa9, 0xc0, 0xaa, 0xe8, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xe8, 0xe8, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_x = 0xff;
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_adc() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x69, 0xfa, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0x5;
    cpu.status_flags.set_carry(true);
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x0);
    assert!(cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_carry());
    assert!(!cpu.status_flags.get_overflow());
}

#[test]
fn test_and() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x29, 0xf7, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0xff;
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xf7);
    assert!(!cpu.status_flags.get_zero());
    assert!(cpu.status_flags.get_negative());
}

#[test]
fn test_asl() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x0A, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0b1010_1010;
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b1010_1010 << 1);
    assert!(!cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_carry());
}

#[test]
fn test_asl_absolute() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x0E, 0x04, 0x80, 0x00, 0b1110_1010])
        .unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.bus.read(0x8004).unwrap(), 0b1110_1010 << 1);
    assert!(!cpu.status_flags.get_zero());
    assert!(cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_carry());
}

#[test]
fn test_bcc_taken() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x90, 0x02, 0xa9, 0x01, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x00);
}

#[test]
fn test_bcc_not_taken() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x90, 0x02, 0xa9, 0x01, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.status_flags.set_carry(true);
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x01);
}

#[test]
fn test_bit_relative() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x24, 0xab, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0b1010_1010;
    cpu.bus.write(0x00ab, 0b1100_1100).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.bus.read(0x00ab).unwrap(), 0b1100_1100);
    assert!(!cpu.status_flags.get_zero());
    assert!(cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_negative());
}

#[test]
fn test_bit_absolute() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x2C, 0xcd, 0xab]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0b0011_0011;
    cpu.bus.write(0xabcd, 0b1100_1100).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.bus.read(0xabcd).unwrap(), 0b1100_1100);
    assert!(cpu.status_flags.get_zero());
    assert!(cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_negative());
}

#[test]
fn test_cmp() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xc9, 0x01, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0x01;
    cpu.run().unwrap();
    assert!(cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
    assert!(!cpu.status_flags.get_carry());
}

#[test]
fn test_decrements() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xc6, 0x00, 0xca, 0x88, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.bus.write(0x0000, 0x10).unwrap();
    cpu.run().unwrap();
    assert!(!cpu.status_flags.get_zero());
    assert!(cpu.status_flags.get_negative());
    assert_eq!(cpu.bus.read(0x0000).unwrap(), 0x0f);
    assert_eq!(cpu.register_x, 0xff);
    assert_eq!(cpu.register_y, 0xff);
}

#[test]
fn test_eor() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x49, 0xab, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0x12;
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xab ^ 0x12);
}

#[test]
fn test_absolute_jmp() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x4C, 0x04, 0x80, 0x00, 0xA9, 0x01, 0x00])
        .unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x01);
}

#[test]
fn test_indirect_jmp() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x6C, 0x03, 0x80, 0x06, 0x80, 0x00, 0xA9, 0x01, 0x00])
        .unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x01);
}

#[test]
fn test_indirect_jmp_bug() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x6C, 0xFF, 0x00, 0x00, 0xA9, 0x01, 0x00])
        .unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.bus.write(0x00FF, 0x04).unwrap();
    cpu.bus.write(0x0000, 0x80).unwrap();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x01);
}

#[test]
fn test_jsr() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x20, 0x04, 0x80, 0x00, 0xA9, 0x01, 0x00])
        .unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x01);
    assert_eq!(cpu.register_sp, 0xFF - 2);
}

#[test]
fn test_ldx_ldy() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xA2, 0x05, 0xA0, 0x06]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0x05);
    assert_eq!(cpu.register_y, 0x06);
    assert!(!cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
}

#[test]
fn test_lsr() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x4A, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0b1010_1010;
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b1010_1010 >> 1);
    assert!(!cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
    assert!(!cpu.status_flags.get_carry());
}

#[test]
fn test_lsr_absolute() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x4E, 0x04, 0x80, 0x00, 0b1110_1011])
        .unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.bus.read(0x8004).unwrap(), 0b1110_1010 >> 1);
    assert!(!cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_carry());
}

#[test]
fn test_ora() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x09, 0xab, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0x12;
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0xab | 0x12);
}

#[test]
fn test_pha() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x48, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0x12;
    cpu.run().unwrap();
    assert_eq!(cpu.register_sp, 0xFF - 1);
    assert_eq!(cpu.bus.read(0x1FF).unwrap(), 0x12);
}

#[test]
fn test_php() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x08, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.status_flags.status = 0b1000_1010;
    cpu.run().unwrap();
    assert_eq!(cpu.register_sp, 0xFF - 1);
    assert_eq!(cpu.bus.read(0x1FF).unwrap(), 0b1011_1010);
}

#[test]
fn test_pla() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x68, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.bus.write(0x1FF, 0x12).unwrap();
    cpu.register_sp = 0xFF - 1;
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x12);
    assert_eq!(cpu.register_sp, 0xFF);
}

#[test]
fn test_plp() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x28, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.bus.write(0x1FF, 0b1011_1010).unwrap();
    cpu.register_sp = 0xFF - 1;
    cpu.run().unwrap();
    assert_eq!(cpu.status_flags.status, 0b1010_1010);
    assert_eq!(cpu.register_sp, 0xFF);
}

#[test]
fn test_rol() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x2A, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0b1010_1011;
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b1010_1011_u8.rotate_left(1));
    assert!(!cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_carry());
}

#[test]
fn test_rol_absolute() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x2E, 0x04, 0x80, 0x00, 0b1110_1011])
        .unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.bus.read(0x8004).unwrap(), 0b1110_1011_u8.rotate_left(1));
    assert!(!cpu.status_flags.get_zero());
    assert!(cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_carry());
}

#[test]
fn test_ror() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x6A, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0b1010_1011;
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0b1010_1011_u8.rotate_right(1));
    assert!(!cpu.status_flags.get_zero());
    assert!(cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_carry());
}

#[test]
fn test_ror_absolute() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x6E, 0x04, 0x80, 0x00, 0b1110_1011])
        .unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(
        cpu.bus.read(0x8004).unwrap(),
        0b1110_1011_u8.rotate_right(1)
    );
    assert!(!cpu.status_flags.get_zero());
    assert!(cpu.status_flags.get_negative());
    assert!(cpu.status_flags.get_carry());
}

#[test]
fn test_rti() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x40, 0x00, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.bus.write_word(0x1FE, 0x8002).unwrap();
    cpu.bus.write(0x1FD, 0b1011_1010).unwrap();
    cpu.register_sp = 0xFF - 3;
    cpu.run().unwrap();
    assert_eq!(cpu.status_flags.status, 0b1010_1010);
    assert_eq!(cpu.register_sp, 0xFF);
    assert_eq!(cpu.register_pc, 0x8002 + 1);
}

#[test]
fn test_rts() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x60, 0x00, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.bus.write_word(0x1FE, 0x8002).unwrap();
    cpu.register_sp = 0xFF - 2;
    cpu.run().unwrap();
    assert_eq!(cpu.register_pc, 0x8002 + 1);
}

#[test]
fn test_sbc() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0xE9, 0xff, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0x12;
    cpu.status_flags.set_carry(true);
    cpu.run().unwrap();
    assert_eq!(cpu.register_a, 0x13);
    assert!(!cpu.status_flags.get_zero());
    assert!(!cpu.status_flags.get_negative());
    assert!(!cpu.status_flags.get_carry());
    assert!(!cpu.status_flags.get_overflow());
}

#[test]
fn test_sta() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x85, 0xab, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_a = 0x12;
    cpu.run().unwrap();
    assert_eq!(cpu.bus.read(0xab).unwrap(), 0x12);
}

#[test]
fn test_stx() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x86, 0xab, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_x = 0x12;
    cpu.run().unwrap();
    assert_eq!(cpu.bus.read(0xab).unwrap(), 0x12);
}

#[test]
fn test_sty() {
    let mut bus = NesBus::new();
    bus.load_rom(&[0x84, 0xab, 0x00]).unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.register_y = 0x12;
    cpu.run().unwrap();
    assert_eq!(cpu.bus.read(0xab).unwrap(), 0x12);
}

#[test]
fn test_subroutines() {
    let mut bus = NesBus::new();
    bus.load_rom(&[
        0x20, 0x09, 0x80, 0x20, 0x0c, 0x80, 0x20, 0x12, 0x80, 0xa2, 0x00, 0x60, 0xe8, 0xe0, 0x05,
        0xd0, 0xfb, 0x60, 0x00,
    ])
    .unwrap();
    let mut cpu = Cpu::new(Box::new(bus));
    cpu.reset();
    cpu.run().unwrap();
    assert_eq!(cpu.register_x, 0x5);
}
