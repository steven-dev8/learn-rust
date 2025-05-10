use std::io;

fn main() {
    let mut value = String::new();

    io::stdin().read_line(&mut value).unwrap();

    let mut total = value.trim().parse::<f64>().unwrap();
    value.clear();

    io::stdin().read_line(&mut value).unwrap();

    total = total / value.trim().parse::<f64>().unwrap();

    println!("{:.3} km/l", total);
}