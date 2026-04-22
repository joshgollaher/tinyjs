pub mod interpreter;
pub mod scope;
mod builtins;
mod bytecode;
// mod emitter;

pub use interpreter::*;
pub use scope::*;