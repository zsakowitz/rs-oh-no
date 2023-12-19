use crate::uint::{Uint, UintZero};
use std::marker::PhantomData;

pub struct MemoryEmpty;
pub struct MemoryWith<A, B>(PhantomData<A>, PhantomData<B>);

pub trait Memory {
    type Dec<I: Uint>: Memory;
    type Inc<I: Uint>: Memory;
    type Get<I: Uint>: Uint;
    type Set<I: Uint, T: Uint>: Memory;
    type Push<T: Uint>: Memory;
    type Pop: Memory;
    type FirstOr<T: Uint>: Uint;

    const VALUE: ReifiedMemory;

    fn reify() -> Vec<usize>;
}

impl Memory for MemoryEmpty {
    type Dec<I: Uint> = <MemoryWith<UintZero, MemoryEmpty> as Memory>::Dec<I>;
    type Inc<I: Uint> = <MemoryWith<UintZero, MemoryEmpty> as Memory>::Inc<I>;
    type Get<I: Uint> = UintZero;
    type Set<I: Uint, T: Uint> = <MemoryWith<UintZero, MemoryEmpty> as Memory>::Set<I, T>;
    type Push<T: Uint> = MemoryWith<T, MemoryEmpty>;
    type Pop = MemoryEmpty;
    type FirstOr<T: Uint> = T;

    const VALUE: ReifiedMemory = ReifiedMemory::None;

    fn reify() -> Vec<usize> {
        Vec::new()
    }
}

impl<A: Uint, B: Memory> Memory for MemoryWith<A, B> {
    type Dec<I: Uint> = I::DecIn<A, B>;
    type Inc<I: Uint> = I::IncIn<A, B>;
    type Get<I: Uint> = I::GetIn<A, B>;
    type Set<I: Uint, T: Uint> = I::SetIn<A, B, T>;
    type Push<T: Uint> = MemoryWith<A, B::Push<T>>;
    type Pop = B;
    type FirstOr<T: Uint> = A;

    const VALUE: ReifiedMemory = ReifiedMemory::Some(A::VALUE, &B::VALUE);

    fn reify() -> Vec<usize> {
        let mut output = B::reify();
        output.insert(0, A::VALUE);
        output
    }
}

#[derive(Debug)]
pub enum ReifiedMemory {
    None,
    Some(usize, &'static ReifiedMemory),
}
