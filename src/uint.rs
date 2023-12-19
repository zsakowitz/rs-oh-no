//! A type-level implementation of unsigned integers. Like with memory buffers, we can't use enums,
//! so we have a `Zero` value and `Next(T)` value, and have them implement a common trait.

use crate::{
    memory::{Memory, MemoryWith},
    tape::TapeTrait,
    Instruction,
};
use std::marker::PhantomData;

/// The number zero.
pub struct UintZero;

/// The number after `T`.
pub struct UintNext<T>(PhantomData<T>);

/// A general unsigned integer.
pub trait Uint {
    /// The number after this one.
    type Next: Uint;

    /// The number before this one. On `UintZero`, returns `0`.
    type Prev: Uint;

    /// Decrements the memory buffer `A:B` at index `Self`.
    type DecIn<A: Uint, B: Memory>: Memory;

    /// Increments the memory buffer `A:B` at index `Self`.
    type IncIn<A: Uint, B: Memory>: Memory;

    /// Gets the value of the memory buffer `A:B` at index `Self`.
    type GetIn<A: Uint, B: Memory>: Uint;

    /// Sets the value of the memory buffer `A:B` at index `Self` to `T`.
    type SetIn<A: Uint, B: Memory, T: Uint>: Memory;

    /// Applies an instruction to a tape while the currently pointed-at value is nonzero.
    type ApplyWhileNonzero<T: TapeTrait, I: Instruction>: TapeTrait;

    /// The runtime-inspectable value of this integer.
    #[cfg(feature = "inspect")]
    const VALUE: usize;
}

impl Uint for UintZero {
    // `Next` is simple; we just wrap the value in `UintNext`.
    type Next = UintNext<Self>;

    // The predecessor of zero is defined to be zero.
    type Prev = UintZero;

    // Decrementing `A:B` at index 0 is just `Dec(A):B`.
    type DecIn<A: Uint, B: Memory> = MemoryWith<A::Prev, B>;

    // Incrementing `A:B` at index 0 is just `Inc(A):B`.
    type IncIn<A: Uint, B: Memory> = MemoryWith<A::Next, B>;

    // Getting `A:B` at index 0 is just `A`.
    type GetIn<A: Uint, B: Memory> = A;

    // Setting `A:B` at index 0 to `T` is just `T:B`.
    type SetIn<A: Uint, B: Memory, T: Uint> = MemoryWith<T, B>;

    // We are zero, so do nothing here.
    type ApplyWhileNonzero<T: TapeTrait, I: Instruction> = T;

    // Self-explanatory.
    #[cfg(feature = "inspect")]
    const VALUE: usize = 0;
}

impl<T: Uint> Uint for UintNext<T> {
    // `Next` is simple; we just wrap the value in `UintNext`.
    type Next = UintNext<Self>;

    // Having a number in `Succ(T)` makes it really easy to get the previous value.
    type Prev = T;

    // Decrementing `A:B` at `(T+1)` is the same as `A:(decrement B at T)`.
    type DecIn<A: Uint, B: Memory> = MemoryWith<A, B::Dec<T>>;

    // Incrementing `A:B` at `(T+1)` is the same as `A:(increment B at T)`.
    type IncIn<A: Uint, B: Memory> = MemoryWith<A, B::Inc<T>>;

    // Getting `A:B` at `(T+1)` is the same as getting `B` at `T`.
    type GetIn<A: Uint, B: Memory> = B::Get<T>;

    // Setting `A:B` at `(T+1)` to `U` is the same as `A:(set B[T] to U)`.
    type SetIn<A: Uint, B: Memory, U: Uint> = MemoryWith<A, B::Set<T, U>>;

    // We know we're nonzero, so apply I to A and call `ApplyWhileNonzero<I>` again.
    type ApplyWhileNonzero<A: TapeTrait, I: Instruction> =
        <I::ApplyTo<A> as TapeTrait>::ApplyWhileNonzero<I>;

    // Fine, this const item does some computation. But you can disable it, so it's fine.
    #[cfg(feature = "inspect")]
    const VALUE: usize = 1 + T::VALUE;
}
