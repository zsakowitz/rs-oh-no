use crate::{
    bool::Bool,
    memory::{Memory, MemoryEmpty, ReifiedMemory},
    uint::{Uint, UintZero},
    Instruction,
};
use std::marker::PhantomData;

pub struct Tape<T, M, I, O>(
    PhantomData<T>,
    PhantomData<M>,
    PhantomData<I>,
    PhantomData<O>,
);

pub type ReifiedTape = (usize, ReifiedMemory, ReifiedMemory, ReifiedMemory);
pub type TapeValue = (usize, Vec<bool>, Vec<bool>, Vec<bool>);

pub type BlankTape<I = MemoryEmpty> = Tape<UintZero, MemoryEmpty, I, MemoryEmpty>;

pub trait TapeTrait {
    type Flip: TapeTrait;
    type Shl: TapeTrait;
    type Shr: TapeTrait;
    type Read: TapeTrait;
    type Write: TapeTrait;
    type ApplyIfTrue<U: Instruction>: TapeTrait;
    type ApplyWhileTrue<U: Instruction>: TapeTrait;
    type Get: Bool;

    const VALUE: ReifiedTape;

    fn reify() -> TapeValue;
}

impl<T: Uint, M: Memory, I: Memory, O: Memory> TapeTrait for Tape<T, M, I, O> {
    type Flip = Tape<T, M::Flip<T>, I, O>;
    type Shl = Tape<T::Prev, M, I, O>;
    type Shr = Tape<T::Next, M, I, O>;
    type Read = Tape<T, M::Set<T, I::FirstOr<M::Get<T>>>, I::Pop, O>;
    type Write = Tape<T, M, I, O::Push<M::Get<T>>>;
    type ApplyIfTrue<U: Instruction> = <Self::Get as Bool>::ChooseTape<U::Apply<Self>, Self>;
    type ApplyWhileTrue<U: Instruction> = <Self::Get as Bool>::ApplyWhileTrue<Self, U>;
    type Get = M::Get<T>;

    const VALUE: ReifiedTape = (T::VALUE, M::VALUE, I::VALUE, O::VALUE);

    fn reify() -> TapeValue {
        (T::VALUE, M::reify(), I::reify(), O::reify())
    }
}
