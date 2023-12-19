//! Defines useful macros.

/// Constructs a type representing a program given brainfuck input. Use Rust-style comments.
#[macro_export]
macro_rules! program {
    (+) => {
        $crate::instruction::Inc
    };

    (-) => {
        $crate::instruction::Dec
    };

    (<) => {
        $crate::instruction::Shl
    };

    (>) => {
        $crate::instruction::Shr
    };

    (.) => {
        $crate::instruction::Read
    };

    (,) => {
        $crate::instruction::Write
    };

    ([]) => {
        $crate::Instruction::Loop<Nop>
    };

    ([ $($x:tt)+ ]) => {
        $crate::instruction::Loop<program!($($x)+)>
    };

    ($a:tt $($b:tt)+) => {
        $crate::instruction::Seq<program!($a), program!($($b)+)>
    }
}

pub use program;
