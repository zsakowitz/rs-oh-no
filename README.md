# rs-oh-no

A brainfuck implementation done purely in the Rust type system, because
computing things at runtime is for the weak-minded.

## How?

It's pretty simple. Instead of actual values, we use types to represent
everything. Instead of functions, we use traits with associated types which take
input generics. Instead of enums, we create a few concrete types and have them
implement a common trait. It's like functional programming, but without enums,
addition, array indexing, concise syntax, or types higher than kind `* -> *`.
That's it!
