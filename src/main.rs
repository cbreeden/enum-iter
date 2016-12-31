#![feature(proc_macro)]
#[macro_use] extern crate enum_iter;

// #[derive(Debug)]
// struct NoDefault;

#[derive(Debug, EnumIterator)]
pub enum Test {
    A,
    B,
    C(u32, u32),
    D { name: String, flag: bool },
    // E(NoDefault),
}

fn main() {
    for variant in Test::enum_iter() {
        println!("{:?}", variant);
    }
}
