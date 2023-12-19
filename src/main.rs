#![recursion_limit = "8"]

use crate::tape::{BlankTape, TapeTrait};
use std::marker::PhantomData;

pub mod bool;
pub mod macros;
pub mod memory;
pub mod tape;
pub mod uint;

pub struct Flip;
pub struct Shl;
pub struct Shr;
pub struct Read;
pub struct Write;
pub struct Seq<A, B>(PhantomData<A>, PhantomData<B>);
pub struct Loop<I>(PhantomData<I>);

pub trait Instruction {
    type Apply<T: TapeTrait>: TapeTrait;
}

impl Instruction for Flip {
    type Apply<T: TapeTrait> = T::Flip;
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
    type Apply<T: TapeTrait> = T::ApplyWhileTrue<I>;
}

fn main() {
    type Step0 = BlankTape;
    type Step1 = <Shr as Instruction>::Apply<Step0>;
    type Step2 = <Flip as Instruction>::Apply<Step1>;
    type Step3 = <Loop<Shl> as Instruction>::Apply<Step2>;

    let x = Step3::reify();

    println!("{x:?}");
}
