//! Defines the seven instruction types.

use crate::tape::TapeTrait;
use std::marker::PhantomData;

/// An instruction which does nothing.
pub struct Nop;

/// An instruction which decrements the currently pointed-at value.
pub struct Dec;

/// An instruction which increments the currently pointed-at value.
pub struct Inc;

/// An instruction which moves the pointer left. Will wrap at the left boundary.
pub struct Shl;

/// An instruction which moves the pointer right.
pub struct Shr;

/// An instruction which reads a byte from input into the currently pointed-at value. Leaves the
/// cell unchanged if no input is left.
pub struct Read;

/// An instruction which writes the currently pointed-at value into the output.
pub struct Write;

/// An instruction which executes two instructions: first A, thenB.
pub struct Seq<A, B>(PhantomData<A>, PhantomData<B>);

/// An instruction which executes its inner instruction while the currently pointed-at cell is
/// nonzero.
pub struct Loop<I>(PhantomData<I>);

/// Allow an instruction to be applied to a tape.
pub trait Instruction {
    /// Applies this instruction to a given tape.
    type ApplyTo<T: TapeTrait>: TapeTrait;
}

impl Instruction for Nop {
    type ApplyTo<T: TapeTrait> = T;
}

impl Instruction for Dec {
    type ApplyTo<T: TapeTrait> = T::Dec;
}

impl Instruction for Inc {
    type ApplyTo<T: TapeTrait> = T::Inc;
}

impl Instruction for Shl {
    type ApplyTo<T: TapeTrait> = T::Shl;
}

impl Instruction for Shr {
    type ApplyTo<T: TapeTrait> = T::Shr;
}

impl Instruction for Read {
    type ApplyTo<T: TapeTrait> = T::Read;
}

impl Instruction for Write {
    type ApplyTo<T: TapeTrait> = T::Write;
}

impl<A: Instruction, B: Instruction> Instruction for Seq<A, B> {
    type ApplyTo<T: TapeTrait> = B::ApplyTo<A::ApplyTo<T>>;
}

impl<I: Instruction> Instruction for Loop<I> {
    type ApplyTo<T: TapeTrait> = T::ApplyWhileNonzero<I>;
}
