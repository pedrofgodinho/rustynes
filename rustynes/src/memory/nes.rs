use crate::memory::Bus;
use crate::EmulationError;
use crate::rom::Rom;


const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x3FFF;
const ROM_START: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;

pub struct NesBus {
    ram: [u8; 0x2000],
    rom: Rom,
}

impl Bus for NesBus {
    fn read(&self, address: u16) -> Result<u8, EmulationError> {
        match address {
            RAM_START..=RAM_END => {
                let mirror = (address - RAM_START) & 0b0000_0111_1111_1111;
                Ok(self.ram[mirror as usize])
            },
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                let _mirror = address & 0b0010_0000_0000_0111;
                Err(EmulationError::InvalidRead)
            },
            ROM_START..=ROM_END => {
                Ok(self.rom.read_prg_rom(address - ROM_START))
            },
            _ => Err(EmulationError::InvalidRead),
        }
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), EmulationError> {
        match address {
            RAM_START..=RAM_END => {
                let mirror = (address - RAM_START) & 0b00000111_11111111;
                self.ram[mirror as usize] = value;
                Ok(())
            },
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                let _mirror = address & 0b00100000_00000111;
                Err(EmulationError::InvalidWrite)
            },
            ROM_START..=ROM_END => {
                Err(EmulationError::InvalidWrite)
            },
            _ => Err(EmulationError::InvalidWrite),
        }
    }


    fn reset(&mut self) {
        self.ram = [0; 0x2000];
    }
}

impl NesBus {
    pub fn new(rom: Rom) -> NesBus {
        NesBus {
            ram: [0; 0x2000],
            rom,
        }
    }
}
