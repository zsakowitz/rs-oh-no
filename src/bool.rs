pub struct True;
pub struct False;

pub trait Bool {
    type Not: Bool;
    type And<T: Bool>: Bool;
    type Or<T: Bool>: Bool;
    type Choose<A: Bool, B: Bool>: Bool;

    const VALUE: bool;
}

impl Bool for True {
    type Not = False;
    type And<T: Bool> = T;
    type Or<T: Bool> = True;
    type Choose<A: Bool, B: Bool> = A;

    const VALUE: bool = true;
}

impl Bool for False {
    type Not = True;
    type And<T: Bool> = False;
    type Or<T: Bool> = T;
    type Choose<A: Bool, B: Bool> = B;

    const VALUE: bool = false;
}
