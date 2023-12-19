//! A brainfuck implementation using only Rust's trait system.

use crate::{
    instruction::Instruction,
    tape::{BlankTape, TapeTrait},
};

pub mod bool;
pub mod instruction;
pub mod macros;
pub mod memory;
pub mod tape;
pub mod uint;

fn main() {
    type Program = program!(
        > + + + // add 3 to cell #1
        [ // while cell #1 is nonzero
            < ++++ // add 4 to cell #0
            > - // remove 1 from cell #1
        ]
    );

    type Result = <Program as Instruction>::Apply<BlankTape>;
    println!("{:?}", Result::reify());
}
