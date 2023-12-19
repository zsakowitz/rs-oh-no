#[macro_export]
#[rustfmt::skip]
macro_rules! ty {
    (($($x:tt)+)) => {
        ty!($($x)+)
    };

    (true)  => { $crate::bool::True  };
    (false) => { $crate::bool::False };

    (0u) => { $crate::uint::Uint0 };
    (1u) => { $crate::uint::Uint1 };
    (2u) => { $crate::uint::Uint2 };
    (3u) => { $crate::uint::Uint3 };
    (4u) => { $crate::uint::Uint4 };
    (5u) => { $crate::uint::Uint5 };
    (6u) => { $crate::uint::Uint6 };
    (7u) => { $crate::uint::Uint7 };
    (8u) => { $crate::uint::Uint8 };
    (9u) => { $crate::uint::Uint9 };

    (! $x:tt) => {
        <ty!($x) as $crate::bool::Bool>::Not
    };

    ($x:tt && $y:tt) => {
        <ty!($x) as $crate::bool::Bool>::And<ty!($y)>
    };

    ($x:tt || $y:tt) => {
        <ty!($x) as $crate::bool::Bool>::Or<ty!($y)>
    };

    ($x:tt + $y:tt) => {
        <ty!($x) as $crate::uint::Uint>::Add<ty!($y)>
    };

    ($x:tt - $y:tt) => {
        <ty!($x) as $crate::uint::Uint>::Sub<ty!($y)>
    };

    ($x:tt * $y:tt) => {
        <ty!($x) as $crate::uint::Uint>::Mult<ty!($y)>
    };

    ($x:tt * $y:tt + $z:tt) => {
        <ty!($x) as $crate::uint::Uint>::MultAndAdd<ty!($y), ty!($z)>
    };

    ($x:tt >= $y:tt) => {
        <ty!($x) as $crate::uint::Uint>::IsGTE<ty!($y)>
    };

    ($x:tt > $y:tt) => {
        <ty!($x) as $crate::uint::Uint>::IsGT<ty!($y)>
    };

    ($x:tt <= $y:tt) => {
        <ty!($x) as $crate::uint::Uint>::IsLTE<ty!($y)>
    };

    ($x:tt < $y:tt) => {
        <ty!($x) as $crate::uint::Uint>::IsLT<ty!($y)>
    };
}

#[macro_export]
macro_rules! val {
    (bool: $($x:tt)+) => {
        <ty!($($x)+) as $crate::bool::Bool>::VALUE
    };

    (uint: $($x:tt)+) => {
        <ty!($($x)+) as $crate::uint::Uint>::VALUE
    };
}

#[macro_export]
macro_rules! program {
    (+) => {
        $crate::instruction::Inc
    };

    (-) => {
        $crate::instruction::Dec
    };

    (<) => {
        $crate::instruction::Shl
    };

    (>) => {
        $crate::instruction::Shr
    };

    (.) => {
        $crate::instruction::Read
    };

    (,) => {
        $crate::instruction::Write
    };

    ([ $($x:tt)+ ]) => {
        $crate::instruction::Loop<program!($($x)+)>
    };

    ($a:tt $($b:tt)+) => {
        $crate::instruction::Seq<program!($a), program!($($b)+)>
    }
}

pub use program;
pub use ty;
pub use val;
