use std::io;

static PI: f64 = 3.14159;

fn main() {
    let mut raio = String::new();

    io::stdin().read_line(&mut raio).unwrap();

    let raio = raio.trim().parse::<f64>().unwrap();

    let volume = (4.0/3.0) * PI * raio.powi(3);

    println!("VOLUME = {:.3}", volume);
}
