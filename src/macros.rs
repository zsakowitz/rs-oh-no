//! Defines useful macros.

/// Constructs a type representing a program given brainfuck input. Use Rust-style comments instead
/// of normal brainfuck comments inside of the program input.
#[macro_export]
macro_rules! program {
    (+) => { $crate::instruction::Inc };

    (-) => { $crate::instruction::Dec };

    (<) => { $crate::instruction::Shl };

    // `<<` has to be matched because Rust's `tt` matcher considers it to be one token.
    (<<) => { $crate::instruction::Seq<$crate::instruction::Shl, $crate::instruction::Shl> };

    (>) => { $crate::instruction::Shr };

    // `>>` has to be matched because Rust's `tt` matcher considers it to be one token.
    (>>) => { $crate::instruction::Seq<$crate::instruction::Shr, $crate::instruction::Shr> };

    // `<-` has to be matched because Rust's `tt` matcher considers it to be one token.
    (<-) => { $crate::instruction::Seq<$crate::instruction::Shl, $crate::instruction::Dec> };

    // `->` has to be matched because Rust's `tt` matcher considers it to be one token.
    (->) => { $crate::instruction::Seq<$crate::instruction::Dec, $crate::instruction::Shr> };

    (,) => { $crate::instruction::Read };

    (.) => { $crate::instruction::Write };

    // `..` has to be matched because Rust's `tt` matcher considers it to be one token.
    (..) => { $crate::instruction::Seq<$crate::instruction::Write, $crate::instruction::Write> };

    // `...` has to be matched because Rust's `tt` matcher considers it to be one token.
    (...) => { $crate::instruction::Seq<$crate::instruction::Write, program!(..)> };

    ([ $($x:tt)+ ]) => { $crate::instruction::Loop<program!($($x)+)> };

    ($a:tt $($b:tt)+) => { $crate::instruction::Seq<program!($a), program!($($b)+)> };
}

pub use program;
