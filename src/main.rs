#![feature(proc_macro)]
#[macro_use] extern crate enum_iter;

#[derive(Debug)]
struct NoDefault;

#[derive(Debug, EnumIterator)]
pub enum Test {
    A,
    B(NoDefault),
}

fn main() {
    for variant in Test::enum_iter() {
        println!("{:?}", variant);
    }
}
