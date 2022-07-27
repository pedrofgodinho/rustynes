use egui::Memory;
use crate::memory::Bus;
use crate::memory::nes::NesBus;

#[test]
fn test_load_rom() {
    let mut memory = NesBus::new();
    memory.load_rom(&[0x00, 0x01, 0x02, 0x03]).unwrap();
    assert_eq!(memory.read(0x8000).unwrap(), 0x00);
    assert_eq!(memory.read(0x8001).unwrap(), 0x01);
    assert_eq!(memory.read(0x8002).unwrap(), 0x02);
    assert_eq!(memory.read(0x8003).unwrap(), 0x03);
}

#[test]
fn test_write_read() {
    let mut memory = NesBus::new();
    memory.write(0x1234, 0xab).unwrap();
    assert_eq!(memory.read(0x1234).unwrap(), 0xab);
}

#[test]
fn test_write_word() {
    let mut memory = NesBus::new();
    memory.write_word(0x1234, 0xabcd).unwrap();
    assert_eq!(memory.read(0x1234).unwrap(), 0xcd);
    assert_eq!(memory.read(0x1235).unwrap(), 0xab);
    assert_eq!(memory.read_word(0x1234).unwrap(), 0xabcd);
}
