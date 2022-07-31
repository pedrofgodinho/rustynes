pub mod cpu;
pub mod memory;
pub mod ui;
pub mod rom;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulationError {
    #[error("Invalid opcode: {0:X}")]
    InvalidOpcode(u8),
    #[error("PC out of bounds: {0}")]
    PcOutOfBounds(u16),
    #[error("Rom to large to fit in memory")]
    RomTooLarge,
    #[error("Invalid read")]
    InvalidRead,
    #[error("Invalid write")]
    InvalidWrite,
    #[error("Addressing mode does not refer to memory address")]
    UnsuportedAddressingMode,
    #[error("Cannot step while halted")]
    Halted,

    #[error("Invalid address")]
    InvalidAddress(u16),
}
