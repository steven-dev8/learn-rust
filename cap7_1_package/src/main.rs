use crate::smartphone::galaxy::S21;
use crate::smartphone::Generic;

pub mod smartphone;

fn main() {
    let smart1 = S21{};
    println!("My phone is {:?}", smart1);

    let smart2 = Generic{};
    println!("My phone is {:?}", smart2);
}