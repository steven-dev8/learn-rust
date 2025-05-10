use std::io;

fn main() {
    loop {
        let mut string = String::new();

        io::stdin()
        .read_line(&mut string)
        .unwrap(); // unwrap extrai um valor de Option e Result

        let string = string.trim();

        let r_string: String = string.chars().rev().collect();
        println!("{r_string}")
    }
}
