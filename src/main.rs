#[macro_use]
extern crate enum_primitive_derive;

pub mod instruction;
pub mod vm;

fn main() {
    let machine = vm::VM::new();
    println!("Hello, world!");
}
