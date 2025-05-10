use std::io;

static PI: f64 = 3.14159;

fn main() {
    let mut values = String::new();

    io::stdin().read_line(&mut values).unwrap();

    let values: Vec<f64> = values.trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    println!("TRIANGULO: {:.3}", (values[0] * values[2]) / 2.0);
    println!("CIRCULO: {:.3}", PI * values[2].powi(2));
    println!("TRAPEZIO: {:.3}", (values[0] + values[1]) * values[2] / 2.0);
    println!("QUADRADO: {:.3}", values[1].powi(2));
    println!("RETANGULO: {:.3}", values[0] * values[1]);
}
