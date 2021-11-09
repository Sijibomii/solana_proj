use thiserror::Error;
use solana_program::program_error::ProgramError;
#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    //the invalid instruction is a type of our self-made escrow error
    InvalidInstruction,
}
//thid implements the std::convert::From<error::EscrowError> trait that helps to convert errors from our custom
//Escrow errors to program errors
impl From<EscrowError> for ProgramError {
  fn from(e: EscrowError) -> Self {
      ProgramError::Custom(e as u32)
  }
}