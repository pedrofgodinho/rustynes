use crate::EmulationError;

#[cfg(test)]
mod test;

pub struct Memory {
    program_ram: [u8; 0x2000],
    program_rom: [u8; 0x8000],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
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
        Ok(())
    }

    pub fn read(&self, address: u16) -> u8 {
        let (region, idx) = self.virtual_address_to_slice_and_index(address);
        region[idx]
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), EmulationError> {
        let (region, idx) = self.virtual_address_to_slice_and_index_mut(address)?;
        region[idx] = value;
        Ok(())
    }

    pub fn read_word(&self, address: u16) -> u16 {
        u16::from_le_bytes([self.read(address), self.read(address + 1)])
    }

    pub fn write_word(&mut self, address: u16, value: u16) -> Result<(), EmulationError> {
        self.write(address, value as u8)?;
        self.write(address + 1, (value >> 8) as u8)?;
        Ok(())
    }

    fn virtual_address_to_slice_and_index_mut(&mut self, address: u16) -> Result<(&mut [u8], usize), EmulationError> {
        match address {
            0x0000..=0x1fff => Ok((&mut self.program_ram, address as usize)),
            0x8000..=0xffff => Ok((&mut self.program_rom, address as usize - 0x8000)),
            //0x8000..=0xffff => Err(EmulationError::InvalidWrite),
            _ => todo!(),
        }
    }

    fn virtual_address_to_slice_and_index(&self, address: u16) -> (&[u8], usize) {
        match address {
            0x0000..=0x1fff => (&self.program_ram, address as usize),
            0x8000..=0xffff => (&self.program_rom, address as usize - 0x8000),
            _ => todo!(),
        }
    }
}