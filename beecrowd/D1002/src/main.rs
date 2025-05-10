use std::io;

static PI: f64 = 3.14159;

fn main() {
    let mut raio = String::new();

    io::stdin()
    .read_line(&mut raio)
    .expect("Error");

    let raio: f64 = raio.trim().parse().unwrap();

    let area = PI * raio.powi(2);

    println!("A={:.4}", area);
}