use std::marker::PhantomData;

pub mod bool;
pub mod macros;
pub mod uint;

pub struct Flip;
pub struct Shl;
pub struct Shr;
pub struct Read;
pub struct Write;
pub struct Seq<A, B>(PhantomData<A>, PhantomData<B>);
pub struct Loop<T>(PhantomData<T>);

trait Instruction {}

impl Instruction for Flip {}
impl Instruction for Shl {}
impl Instruction for Shr {}
impl Instruction for Read {}
impl Instruction for Write {}
impl<A: Instruction, B: Instruction> Instruction for Seq<A, B> {}
impl<T: Instruction> Instruction for Loop<T> {}

fn main() {
    println!("Hello, world!");
}
