use crate::EmulationError;

pub mod nes;
#[cfg(test)]
mod test;
pub mod test_game;

pub trait Bus {
    fn read(&self, address: u16) -> Result<u8, EmulationError>;
    fn write(&mut self, address: u16, value: u8) -> Result<(), EmulationError>;
    fn read_word(&self, address: u16) -> Result<u16, EmulationError>;
    fn write_word(&mut self, address: u16, value: u16) -> Result<(), EmulationError>;
    fn reset(&mut self);
}
