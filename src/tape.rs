use crate::{
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
pub type TapeValue = (usize, Vec<usize>, Vec<usize>, Vec<usize>);

pub type BlankTape<I = MemoryEmpty> = Tape<UintZero, MemoryEmpty, I, MemoryEmpty>;

pub trait TapeTrait {
    type Dec: TapeTrait;
    type Inc: TapeTrait;
    type Shl: TapeTrait;
    type Shr: TapeTrait;
    type Read: TapeTrait;
    type Write: TapeTrait;
    type ApplyWhileNonzero<U: Instruction>: TapeTrait;
    type Get: Uint;

    const VALUE: ReifiedTape;

    fn reify() -> TapeValue;
}

impl<T: Uint, M: Memory, I: Memory, O: Memory> TapeTrait for Tape<T, M, I, O> {
    type Dec = Tape<T, M::Dec<T>, I, O>;
    type Inc = Tape<T, M::Inc<T>, I, O>;
    type Shl = Tape<T::Prev, M, I, O>;
    type Shr = Tape<T::Next, M, I, O>;
    type Read = Tape<T, M::Set<T, I::FirstOr<M::Get<T>>>, I::Pop, O>;
    type Write = Tape<T, M, I, O::Push<M::Get<T>>>;
    type ApplyWhileNonzero<U: Instruction> = <Self::Get as Uint>::ApplyWhileNonzero<Self, U>;
    type Get = M::Get<T>;

    const VALUE: ReifiedTape = (T::VALUE, M::VALUE, I::VALUE, O::VALUE);

    fn reify() -> TapeValue {
        (T::VALUE, M::reify(), I::reify(), O::reify())
    }
}
