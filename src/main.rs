use crate::tape::{BlankTape, TapeTrait};
use std::marker::PhantomData;

pub mod bool;
pub mod macros;
pub mod memory;
pub mod tape;
pub mod uint;

pub struct Dec;
pub struct Inc;
pub struct Shl;
pub struct Shr;
pub struct Read;
pub struct Write;
pub struct Seq<A, B>(PhantomData<A>, PhantomData<B>);
pub struct Loop<I>(PhantomData<I>);

pub trait Instruction {
    type Apply<T: TapeTrait>: TapeTrait;
}

impl Instruction for Dec {
    type Apply<T: TapeTrait> = T::Dec;
}

impl Instruction for Inc {
    type Apply<T: TapeTrait> = T::Inc;
}

impl Instruction for Shl {
    type Apply<T: TapeTrait> = T::Shl;
}

impl Instruction for Shr {
    type Apply<T: TapeTrait> = T::Shr;
}

impl Instruction for Read {
    type Apply<T: TapeTrait> = T::Read;
}

impl Instruction for Write {
    type Apply<T: TapeTrait> = T::Write;
}

impl<A: Instruction, B: Instruction> Instruction for Seq<A, B> {
    type Apply<T: TapeTrait> = B::Apply<A::Apply<T>>;
}

impl<I: Instruction> Instruction for Loop<I> {
    type Apply<T: TapeTrait> = T::ApplyWhileNonzero<I>;
}

macro_rules! program {
    (+) => {
        Inc
    };

    (-) => {
        Dec
    };

    (<) => {
        Shl
    };

    (>) => {
        Shr
    };

    (.) => {
        Read
    };

    (,) => {
        Write
    };

    ([ $($x:tt)+ ]) => {
        Loop<program!($($x)+)>
    };

    ($a:tt $($b:tt)+) => {
        Seq<program!($a), program!($($b)+)>
    }
}

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
