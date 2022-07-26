pub mod cpu;
pub mod memory;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulationError {
    #[error("Invalid opcode: {0:X}")]
    InvalidOpcode(u8),
    #[error("PC out of bounds: {0}")]
    PcOutOfBounds(u16),
    #[error("Rom to large to fit in memory")]
    RomTooLarge,
    #[error("Tried to write to read-only memory")]
    InvalidWrite,
}