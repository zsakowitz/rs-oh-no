use crate::{
    bool::{Bool, False, True},
    memory::{Memory, MemoryWith},
    tape::TapeTrait,
    Instruction,
};
use std::marker::PhantomData;

pub struct UintZero;
pub struct UintNext<T>(PhantomData<T>);

pub trait Uint {
    type Next: Uint;
    type Prev: Uint;
    type Add<Rhs: Uint>: Uint;
    type Sub<Rhs: Uint>: Uint;
    type RevSub<Lhs: Uint>: Uint;
    type MultAndAdd<Rhs: Uint, Extra: Uint>: Uint;
    type Mult<Rhs: Uint>: Uint;
    type IsZero: Bool;
    type IsGT<Rhs: Uint>: Bool;
    type IsGTE<Rhs: Uint>: Bool;
    type IsLT<Rhs: Uint>: Bool;
    type IsLTE<Rhs: Uint>: Bool;
    type DecIn<A: Uint, B: Memory>: Memory;
    type IncIn<A: Uint, B: Memory>: Memory;
    type GetIn<A: Uint, B: Memory>: Uint;
    type SetIn<A: Uint, B: Memory, T: Uint>: Memory;
    type ApplyWhileNonzero<T: TapeTrait, I: Instruction>: TapeTrait;

    const VALUE: usize;
}

impl Uint for UintZero {
    type Next = UintNext<Self>;
    type Prev = UintZero;
    type Add<Rhs: Uint> = Rhs;
    type Sub<Rhs: Uint> = UintZero;
    type RevSub<Lhs: Uint> = Lhs;
    type MultAndAdd<Rhs: Uint, Extra: Uint> = Extra;
    type Mult<Rhs: Uint> = UintZero;
    type IsZero = True;
    type IsGT<Rhs: Uint> = False;
    type IsGTE<Rhs: Uint> = Rhs::IsZero;
    type IsLT<Rhs: Uint> = <Rhs::IsZero as Bool>::Not;
    type IsLTE<Rhs: Uint> = True;
    type DecIn<A: Uint, B: Memory> = MemoryWith<A::Prev, B>;
    type IncIn<A: Uint, B: Memory> = MemoryWith<A::Next, B>;
    type GetIn<A: Uint, B: Memory> = A;
    type SetIn<A: Uint, B: Memory, T: Uint> = MemoryWith<T, B>;
    type ApplyWhileNonzero<T: TapeTrait, I: Instruction> = T;

    const VALUE: usize = 0;
}

impl<T: Uint> Uint for UintNext<T> {
    type Next = UintNext<Self>;
    type Prev = T;
    type Add<Rhs: Uint> = UintNext<T::Add<Rhs>>;
    type Sub<Rhs: Uint> = <Rhs::Prev as Uint>::RevSub<T>;
    type RevSub<Lhs: Uint> = Lhs::Sub<Self>;
    type MultAndAdd<Rhs: Uint, Extra: Uint> =
        <Rhs::Prev as Uint>::MultAndAdd<Self, Extra::Add<Self>>;
    type Mult<Rhs: Uint> = Self::MultAndAdd<Rhs, UintZero>;
    type IsZero = False;
    type IsGT<Rhs: Uint> = <Rhs::IsZero as Bool>::Choose<True, T::IsGT<Rhs::Prev>>;
    type IsGTE<Rhs: Uint> = <Rhs::IsZero as Bool>::Choose<True, T::IsGTE<Rhs::Prev>>;
    type IsLT<Rhs: Uint> = <Rhs::IsZero as Bool>::Choose<False, T::IsLT<Rhs::Prev>>;
    type IsLTE<Rhs: Uint> = <Rhs::IsZero as Bool>::Choose<False, T::IsLTE<Rhs::Prev>>;
    type DecIn<A: Uint, B: Memory> = MemoryWith<A, B::Dec<T>>;
    type IncIn<A: Uint, B: Memory> = MemoryWith<A, B::Inc<T>>;
    type GetIn<A: Uint, B: Memory> = B::Get<T>;
    type SetIn<A: Uint, B: Memory, U: Uint> = MemoryWith<A, B::Set<T, U>>;
    type ApplyWhileNonzero<A: TapeTrait, I: Instruction> =
        <I::Apply<A> as TapeTrait>::ApplyWhileNonzero<I>;

    const VALUE: usize = 1 + T::VALUE;
}

pub type Uint0 = UintZero;
pub type Uint1 = UintNext<Uint0>;
pub type Uint2 = UintNext<Uint1>;
pub type Uint3 = UintNext<Uint2>;
pub type Uint4 = UintNext<Uint3>;
pub type Uint5 = UintNext<Uint4>;
pub type Uint6 = UintNext<Uint5>;
pub type Uint7 = UintNext<Uint6>;
pub type Uint8 = UintNext<Uint7>;
pub type Uint9 = UintNext<Uint8>;
