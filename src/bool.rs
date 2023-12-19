use crate::{tape::TapeTrait, Instruction};

pub struct True;
pub struct False;

pub trait Bool {
    type Not: Bool;
    type And<T: Bool>: Bool;
    type Or<T: Bool>: Bool;
    type Choose<A: Bool, B: Bool>: Bool;
    type ChooseTape<A: TapeTrait, B: TapeTrait>: TapeTrait;
    type ApplyWhileTrue<A: TapeTrait, I: Instruction>: TapeTrait;

    const VALUE: bool;
}

impl Bool for True {
    type Not = False;
    type And<T: Bool> = T;
    type Or<T: Bool> = True;
    type Choose<A: Bool, B: Bool> = A;
    type ChooseTape<A: TapeTrait, B: TapeTrait> = A;
    type ApplyWhileTrue<A: TapeTrait, I: Instruction> =
        <I::Apply<A> as TapeTrait>::ApplyWhileTrue<I>;

    const VALUE: bool = true;
}

impl Bool for False {
    type Not = True;
    type And<T: Bool> = False;
    type Or<T: Bool> = T;
    type Choose<A: Bool, B: Bool> = B;
    type ChooseTape<A: TapeTrait, B: TapeTrait> = B;
    type ApplyWhileTrue<A: TapeTrait, I: Instruction> = A;

    const VALUE: bool = false;
}
