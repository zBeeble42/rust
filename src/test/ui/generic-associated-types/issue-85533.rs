// check-pass

#![feature(generic_associated_types)]

trait Trait {
    type Assoc<'a>;
}

type AssocAlias<'a, T> = <T as Trait>::Assoc<'a>;

fn works<'a, T: Trait>(entity: T::Assoc<'a>) -> &'a i32 {&5}
fn fails<'a, T: Trait>(entity: AssocAlias<'a, T>) -> &'a i32 {&5}

fn main() {}
