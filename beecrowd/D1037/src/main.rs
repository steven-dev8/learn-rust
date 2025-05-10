use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().parse::<f64>().unwrap();

    if input >= 0.0 && input <= 25.0 {
        println!("Intervalo [0,25]");
    } else if input > 25.0 && input <= 50.0 {
        println!("Intervalo (25,50]");
    } else if input > 50.0 && input <= 75.0 {
        println!("Intervalo (50,75]");
    } else if input > 75.0 && input <= 100.0 {
        println!("Intervalo (75,100]");
    } else {
        println!("Fora de intervalo");
    };
}
