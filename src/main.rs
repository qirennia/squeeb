#[macro_use]
extern crate enum_primitive_derive;

mod instruction;
mod vm;

fn main() {
    let machine = vm::VM::new();
    println!("Hello, world!");
}
