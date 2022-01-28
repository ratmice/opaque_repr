use enum_extra::NonZeroRepr;
use enum_extra_derive::NonZeroRepr;
use strum::EnumMetadata;
use strum_macros::EnumMetadata;

// This will check that none of the discriminants is zero.
#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u8)]
#[allow(unused)]
enum Foo {
    Bar = 1 << 0,
    Baz = 1 << 1,
}

fn main() {}
