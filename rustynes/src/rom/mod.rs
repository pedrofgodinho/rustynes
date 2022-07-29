use thiserror::Error;

const NES_MAGIC: &[u8; 4] = b"NES\x1A";
const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Error, Debug)]
pub enum RomError{
    #[error("Invalid header")]
    InvalidFileFormat,
    #[error("Invalid ines version:  only version 1 is supported")]
    InvalidInesVersion,
    #[error("Invalid file size")]
    InvalidFileSize,
}

pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub struct Rom {
    prg_rom: [u8; 0x8000],
    chr_rom: Vec<u8>,
    mapper: u8,
    mirroring: Mirroring,
    mirror_prg_rom: bool,
}

impl Rom {
    pub fn new(raw: &[u8]) -> Result<Rom, RomError> {
        if raw.len() < 16 {
            return Err(RomError::InvalidFileFormat);
        }

        if &raw[0..4] != NES_MAGIC {
            return Err(RomError::InvalidFileFormat)
        }

        let mapper = (raw[7] & 0b1111_0000) | (raw[6] >> 4);
        let ines_ver = (raw[7]) & 0b11;
        if ines_ver != 0 {
            return Err(RomError::InvalidInesVersion);
        }

        let four_screen = raw[6] & 0b1000 != 0;
        let vertical_mirroring = raw[6] & 0b1 != 0;
        let mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw[5] as usize * CHR_ROM_PAGE_SIZE;

        let has_trainer = raw[6] & 0b100 != 0;

        let prg_rom_start = 16 + if has_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        if raw.len() < prg_rom_start + prg_rom_size || raw.len() < chr_rom_start + chr_rom_size {
            return Err(RomError::InvalidFileSize);
        }

        let mut prg_rom = [0; 0x8000];
        prg_rom[0..prg_rom_size].copy_from_slice(&raw[prg_rom_start..prg_rom_start + prg_rom_size]);
        let mirror_prg_rom = prg_rom_size == 0x4000;

        Ok(Rom{
            prg_rom,
            chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper,
            mirroring,
            mirror_prg_rom,
        })
    }

    pub fn read_prg_rom(&self, address: u16) -> u8 {
        if self.mirror_prg_rom {
            self.prg_rom[address as usize % 0x4000]
        } else {
            self.prg_rom[address as usize]
        }
    }
}