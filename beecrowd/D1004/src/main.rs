use std::io;

fn main() {
    let mut val1 = String::new();
    let mut val2 = String::new();

    io::stdin()
    .read_line(&mut val1)
    .expect("Error");

    io::stdin()
    .read_line(&mut val2)
    .expect("Error");

    let val1: i64 = val1.trim().parse().unwrap();
    let val2: i64 = val2.trim().parse().unwrap();

    println!("PROD = {}", val1 * val2);
}