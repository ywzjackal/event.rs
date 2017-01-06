#![allow(dead_code)]
mod holder_ref;
mod holder_mut;
mod holder;

pub use holder_ref::*;
pub use holder_mut::*;
pub use holder::*;

mod test;