use crate::memory::Bus;
use crate::EmulationError;

pub struct NesBus {
    program_ram: [u8; 0x2000],
    program_rom: [u8; 0x8000],
}

impl Bus for NesBus {
    fn read(&self, address: u16) -> Result<u8, EmulationError> {
        let (region, idx) = self.virtual_address_to_slice_and_index(address)?;
        Ok(region[idx])
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), EmulationError> {
        let (region, idx) = self.virtual_address_to_slice_and_index_mut(address)?;
        region[idx] = value;
        Ok(())
    }

    fn read_word(&self, address: u16) -> Result<u16, EmulationError> {
        Ok(u16::from_le_bytes([
            self.read(address)?,
            self.read(address + 1)?,
        ]))
    }

    fn write_word(&mut self, address: u16, value: u16) -> Result<(), EmulationError> {
        self.write(address, value as u8)?;
        self.write(address + 1, (value >> 8) as u8)?;
        Ok(())
    }

    fn reset(&mut self) {
        self.program_ram = [0; 0x2000];
    }
}

impl NesBus {
    pub fn new() -> NesBus {
        NesBus {
            program_ram: [0; 0x2000],
            program_rom: [0; 0x8000],
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), EmulationError> {
        if rom.len() > 0x8000 {
            return Err(EmulationError::RomTooLarge);
        }
        for (i, byte) in rom.iter().enumerate() {
            self.program_rom[i] = *byte;
        }
        self.write_word(0xFFFC, 0x8000).unwrap();
        Ok(())
    }

    fn virtual_address_to_slice_and_index_mut(
        &mut self,
        address: u16,
    ) -> Result<(&mut [u8], usize), EmulationError> {
        match address {
            0x0000..=0x1fff => Ok((&mut self.program_ram, address as usize)),
            0x8000..=0xffff => Ok((&mut self.program_rom, address as usize - 0x8000)),
            //0x8000..=0xffff => Err(EmulationError::InvalidWrite),
            _ => Err(EmulationError::InvalidAddress(address)),
        }
    }

    fn virtual_address_to_slice_and_index(
        &self,
        address: u16,
    ) -> Result<(&[u8], usize), EmulationError> {
        match address {
            0x0000..=0x1fff => Ok((&self.program_ram, address as usize)),
            0x8000..=0xffff => Ok((&self.program_rom, address as usize - 0x8000)),
            _ => Err(EmulationError::InvalidAddress(address)),
        }
    }
}

impl Default for NesBus {
    fn default() -> NesBus {
        NesBus::new()
    }
}
