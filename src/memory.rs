//! Defines memory types used in this crate.
//!
//! Because types are immutable, we store memory in a Haskell-style list, with a nil element called
//! [`MemoryEmpty`] and an x:xs element called [`MemoryWith`].
//!
//! It'd be nice if we could use enums to represent these two states, but you can't match on enums
//! in the Rust type system, so we used two structs and have them implement a common trait called
//! [`Memory`].

use crate::uint::{Uint, UintZero};
use std::marker::PhantomData;

/// An empty memory buffer.
pub struct MemoryEmpty;

/// A memory buffer with a head element and a tail.
///
/// The element `A` is the first element of this memory buffer, and the element `B` is presumed to
/// be another memory buffer.
pub struct MemoryWith<A, B>(PhantomData<A>, PhantomData<B>);

/// A general memory buffer. Implemented as a trait because we can't use enums.
pub trait Memory {
    /// Decrements the value at a given index.
    type Dec<I: Uint>: Memory;

    /// Increments the value at a given index.
    type Inc<I: Uint>: Memory;

    /// Gets the value at a given index.
    type Get<I: Uint>: Uint;

    /// Sets the value at a given index.
    type Set<I: Uint, T: Uint>: Memory;

    /// Pushes a value to the end of the buffer, returning a new buffer.
    type Push<T: Uint>: Memory;

    /// Gets the head of this buffer, returning `T` if it doesn't exist.
    type HeadOr<T: Uint>: Uint;

    /// Gets the tail of this memory buffer, that is, everything except the first value.
    ///
    /// On empty buffers, returns the empty buffer [`MemoryEmpty`].
    type Tail: Memory;

    /// The reified value of this memory buffer, computed at compile time. Represented as an enum
    /// because we can't use `Vec` in const items.
    #[cfg(feature = "inspect")]
    const VALUE: ReifiedMemory;

    /// The value of this memory buffer, computed at runtime. Represented as a `Vec` for
    /// convenience.
    #[cfg(feature = "inspect")]
    fn reify() -> Vec<usize>;
}

impl Memory for MemoryEmpty {
    // On empty memory, we first extend the buffer into 0:nil before decrementing the vaue.
    type Dec<I: Uint> = <MemoryWith<UintZero, MemoryEmpty> as Memory>::Dec<I>;

    // On empty memory, we first extend the buffer into 0:nil before incrementing the value.
    type Inc<I: Uint> = <MemoryWith<UintZero, MemoryEmpty> as Memory>::Inc<I>;

    // An empty buffer contains all zeroes.
    type Get<I: Uint> = UintZero;

    // On empty memory, we first extend the buffer into 0:nil before setting the value.
    type Set<I: Uint, T: Uint> = <MemoryWith<UintZero, MemoryEmpty> as Memory>::Set<I, T>;

    // nil.push(T) => T:nil
    type Push<T: Uint> = MemoryWith<T, MemoryEmpty>;

    // (head nil) or T => T
    type HeadOr<T: Uint> = T;

    // tail nil => nil
    type Tail = MemoryEmpty;

    #[cfg(feature = "inspect")]
    const VALUE: ReifiedMemory = ReifiedMemory::None;

    #[cfg(feature = "inspect")]
    fn reify() -> Vec<usize> {
        Vec::new()
    }
}

impl<A: Uint, B: Memory> Memory for MemoryWith<A, B> {
    // Because we can't directly match on `I`, which we need to do to decide whether we should
    // decrement `A` or continue into `B`, we delegate the decrementing to the index itself.
    //
    // Same logic applies for `Inc`, `Get`, and `Set`.

    type Dec<I: Uint> = I::DecIn<A, B>;
    type Inc<I: Uint> = I::IncIn<A, B>;
    type Get<I: Uint> = I::GetIn<A, B>;
    type Set<I: Uint, T: Uint> = I::SetIn<A, B, T>;

    // Keep the first element, and push `T` onto the end.
    type Push<T: Uint> = MemoryWith<A, B::Push<T>>;

    // head A:B => A
    type HeadOr<T: Uint> = A;

    // tail A:B => B
    type Tail = B;

    /// A runtime-inspectable, const-constructable representation of this type.
    #[cfg(feature = "inspect")]
    const VALUE: ReifiedMemory = ReifiedMemory::Some(A::VALUE, &B::VALUE);

    /// A runtime-inspectable, non-const-constructable representation of this type.
    #[cfg(feature = "inspect")]
    fn reify() -> Vec<usize> {
        let mut output = B::reify();
        output.insert(0, A::VALUE);
        output
    }
}

/// A runtime-inspectable, const-constructable representation of memory.
#[derive(Debug)]
pub enum ReifiedMemory {
    /// An empty memory buffer.
    None,

    /// A memory buffer with a head element and a tail.
    Some(usize, &'static ReifiedMemory),
}
