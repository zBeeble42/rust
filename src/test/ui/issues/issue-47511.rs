// Regression test for #47511: anonymous lifetimes can appear
// unconstrained in a return type, but only if they appear just once
// in the input, as the input to a projection.

#![feature(no_core)]
#![no_core]

fn f(_: X) -> X {
    //~^ ERROR return type references an anonymous lifetime
    //unimplemented!()
    loop {}
}

fn g<'a>(_: X<'a>) -> X<'a> {
    //unimplemented!()
    loop {}
}

fn h<'a>(_: Y<'a, Foo<'a>>) -> X<'a> {
    //unimplemented!()
    loop {}
}

type X<'a> = <&'a () as Trait>::Value;

trait Trait {
    type Value;
}

impl<'a> Trait for &'a () {
    type Value = ();
}

type Y<'a, T> = (T, <&'a () as Trait>::Value);

struct Foo<'a>(&'a ());

fn a<'a>(_: (Foo<'a>, ())) -> () {}
fn b<'a>(_: ((), ())) -> () {}

fn main() {
    let _a: for<'a> fn(Y<'a, ()>) -> X<'a> = a;
    let _b: for<'a> fn(Y<'a, Foo<'a>>) -> X<'a> = b;
}
