use thiserror::Error;
use solana_program::program_error::ProgramError;
#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Expected Amount Mismatch
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    /// Amount Overflow
    #[error("Amount Overflow")]
    AmountOverflow,
}
//thid implements the std::convert::From<error::EscrowError> trait that helps to convert errors from our custom
//Escrow errors to program errors
impl From<EscrowError> for ProgramError {
  fn from(e: EscrowError) -> Self {
      ProgramError::Custom(e as u32)
  }
}

