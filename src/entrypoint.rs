//TUTORIAL:https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/
//-----------------------------------SIDE NOTES----------------------------------------
//accounts can be owned only by programs. Each accounts can hold a SOL owners alone can debit.
//SOL only gets transferred when the transactions has been signed by a pivated key belonging to that account.
// In theory, programs have full autonomy over the accounts they own. It is up to the program's creator 
// to limit this autonomy and up to the users of the program to verify the program's creator has really done so.
//EVERY ACCOUNT belongs to a program!
//All accounts to be read or written to must be passed into the entrypoint function.



//--------------------------END SIDE NOTES--------------------------------------------
//this declares process_instructions as the entry point to our program
//all calls to our program goes through the function declared as an entry point
use solana_program::{
  account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey
};

use crate::processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
  Processor::process(program_id, accounts, instruction_data)
}