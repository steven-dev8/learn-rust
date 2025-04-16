use std::io;

fn main() {
    let arr = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");


    let index: usize = index
    .trim()
    .parse()
    .expect("This is not a valid number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
