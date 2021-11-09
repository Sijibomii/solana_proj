pub mod entrypoint;
pub mod instruction;
pub mod errors;
pub mod processor;
pub mod state;
// lib.rs -> registering modules
// entrypoint.rs -> entrypoint to the program
// instruction.rs -> program API, (de)serializing instruction data
// processor.rs -> program logic
// state.rs -> program objects, (de)serializing state
// error.rs -> program specific errors