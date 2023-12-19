use crate::tape::TapeTrait;
use std::marker::PhantomData;

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
