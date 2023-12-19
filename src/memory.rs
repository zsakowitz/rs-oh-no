use crate::{
    bool::{Bool, False},
    uint::Uint,
};
use std::marker::PhantomData;

pub struct MemoryEmpty;
pub struct MemoryWith<A, B>(PhantomData<A>, PhantomData<B>);

pub trait Memory {
    type Flip<I: Uint>: Memory;
    type Get<I: Uint>: Bool;
    type Set<I: Uint, T: Bool>: Memory;
    type Push<T: Bool>: Memory;
    type Pop: Memory;
    type FirstOr<T: Bool>: Bool;

    const VALUE: ReifiedMemory;

    fn reify() -> Vec<bool>;
}

impl Memory for MemoryEmpty {
    type Flip<I: Uint> = <MemoryWith<False, MemoryEmpty> as Memory>::Flip<I>;
    type Get<I: Uint> = False;
    type Set<I: Uint, T: Bool> = <MemoryWith<False, MemoryEmpty> as Memory>::Set<I, T>;
    type Push<T: Bool> = MemoryWith<T, MemoryEmpty>;
    type Pop = MemoryEmpty;
    type FirstOr<T: Bool> = T;

    const VALUE: ReifiedMemory = ReifiedMemory::None;

    fn reify() -> Vec<bool> {
        Vec::new()
    }
}

impl<A: Bool, B: Memory> Memory for MemoryWith<A, B> {
    type Flip<I: Uint> = I::FlipIn<A, B>;
    type Get<I: Uint> = I::GetIn<A, B>;
    type Set<I: Uint, T: Bool> = I::SetIn<A, B, T>;
    type Push<T: Bool> = MemoryWith<A, B::Push<T>>;
    type Pop = B;
    type FirstOr<T: Bool> = A;

    const VALUE: ReifiedMemory = ReifiedMemory::Some(A::VALUE, &B::VALUE);

    fn reify() -> Vec<bool> {
        let mut output = B::reify();
        output.insert(0, A::VALUE);
        output
    }
}

#[derive(Debug)]
pub enum ReifiedMemory {
    None,
    Some(bool, &'static ReifiedMemory),
}
