use std::io;

fn main() {
    let mut x = String::new();
    let mut y = String::new();
    
    io::stdin()
    .read_line(&mut x)
    .expect("Error");
    
    io::stdin()
    .read_line(&mut y)
    .expect("Error");
    
    let x: i64 = x.trim().parse().expect("Error");
    let y: i64 = y.trim().parse().expect("Error");

    println!("X = {}", x+y);
}
