use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input: Vec<f64> = input.trim().split_whitespace()
                                .map(|x| x.parse::<f64>().unwrap())
                                .collect();

    let (a, b, c) = (input[0], input[1], input[2]);

    let delta = b.powi(2) - 4.0 * a * c;

    let r1 = ((b * -1.0) + delta.sqrt()) / (2.0 * a);
    let r2 = ((b * -1.0) - delta.sqrt()) / (2.0 * a);

    if a == 0.0 || delta < 0.0 {
        println!("Impossivel calcular");
    } else {
        println!("R1 = {:.5}", r1);
        println!("R2 = {:.5}", r2);
    }
}
