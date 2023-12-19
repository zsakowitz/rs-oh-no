//! Contains types used to represent tapes.

use crate::{
    memory::{Memory, MemoryEmpty, ReifiedMemory},
    uint::{Uint, UintZero},
    Instruction,
};
use std::marker::PhantomData;

/// A tape.
///
/// Contains an index into the buffer, the memory itself, and two other memory buffers for input and
/// output data.
pub struct Tape<T, M, I, O>(
    PhantomData<T>,
    PhantomData<M>,
    PhantomData<I>,
    PhantomData<O>,
);

/// A runtime-inspectable, const-constructable representation of a tape.
pub type ReifiedTape = (usize, ReifiedMemory, ReifiedMemory, ReifiedMemory);

/// A runtime-inspectable, non-const-constructable representation of a tape.
pub type TapeValue = (usize, Vec<usize>, Vec<usize>, Vec<usize>);

/// A blank tape.
pub type BlankTape<I = MemoryEmpty> = Tape<UintZero, MemoryEmpty, I, MemoryEmpty>;

/// A trait implemented by `Tape`. Required because associated types can't be constrained on types
/// themselves, only on traits.
pub trait TapeTrait {
    /// This tape, but with the currently pointed-at value decremented by one.
    type Dec: TapeTrait;

    /// This tape, but with the currently pointed-at value incremented by one.
    type Inc: TapeTrait;

    /// This tape, but with the pointer index shifted to the left.
    type Shl: TapeTrait;

    /// This tape, but with the pointer index shifted to the right.
    type Shr: TapeTrait;

    /// This tape, but with the currently pointed-at value being read from the initial value of the
    /// input buffer.
    type Read: TapeTrait;

    /// This tape, but with the output buffer having an additional byte taken from the currently
    /// pointed-at value.
    type Write: TapeTrait;

    /// Applies an instruction while the currently pointed-at value is nonzero.
    type ApplyWhileNonzero<U: Instruction>: TapeTrait;

    /// The currently pointed-at value.
    type Get: Uint;

    /// A runtime-inspectable, const-constructable representation of this type.
    #[cfg(feature = "inspect")]
    const VALUE: ReifiedTape;

    /// A runtime-inspectable, non-const-constructable representation of this type.
    #[cfg(feature = "inspect")]
    fn reify() -> TapeValue;
}

impl<T: Uint, M: Memory, I: Memory, O: Memory> TapeTrait for Tape<T, M, I, O> {
    // `Dec` and `Inc` are quite simple; just decrement and increment the main memory.
    type Dec = Tape<T, M::Dec<T>, I, O>;
    type Inc = Tape<T, M::Inc<T>, I, O>;

    // `Shl` and `Shr` are even simpler; we just take `::Prev` and `::Next`.
    type Shl = Tape<T::Prev, M, I, O>;
    type Shr = Tape<T::Next, M, I, O>;

    // `Read` is just a functional-style `M[T] = I[0] or M[T]`.
    type Read = Tape<T, M::Set<T, I::HeadOr<Self::Get>>, I::Tail, O>;

    // `Write` is also quite easy.
    type Write = Tape<T, M, I, O::Push<Self::Get>>;

    // We delegate `ApplyWhileNonzero` to `Self::Get` so it can be specialized on different `Uint`
    // types.
    type ApplyWhileNonzero<U: Instruction> = <Self::Get as Uint>::ApplyWhileNonzero<Self, U>;

    // `Get` just calls `M[T]`.
    type Get = M::Get<T>;

    // Told you these don't do anything.

    #[cfg(feature = "inspect")]
    const VALUE: ReifiedTape = (T::VALUE, M::VALUE, I::VALUE, O::VALUE);

    #[cfg(feature = "inspect")]
    fn reify() -> TapeValue {
        (T::VALUE, M::reify(), I::reify(), O::reify())
    }
}
