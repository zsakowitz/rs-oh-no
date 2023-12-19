//! A brainfuck implementation using only Rust's trait system.
//!
//! If you run complex programs, you may need to increase the recursion limit. Compiler errors will
//! tell you how.
//!
//! ## How does it work?
//!
//! The overall strategy is to use types as values and traits as functions. Traits can have
//! associated types, which may take generics as parameters and return a type as output. After
//! realizing that, it's pretty much straight shooting, except that we're using a purely functional
//! language. But it works!

#![forbid(missing_docs, clippy::missing_docs_in_private_items)]

use crate::{
    instruction::Instruction,
    tape::{BlankTape, TapeTrait},
};

pub mod instruction;
pub mod macros;
pub mod memory;
pub mod tape;
pub mod uint;

fn main() {
    // Write your program below. The example one outputs "hello world" as a byte sequence.
    type Program = program!(
        ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
    );

    type Result = <Program as Instruction>::ApplyTo<BlankTape>;

    println!("{:?}", <Result as TapeTrait>::reify());
}
