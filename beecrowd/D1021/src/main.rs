use std::io;

fn main() {
    let mut valor = String::new();

    io::stdin().read_line(&mut valor).unwrap();

    let mut valor = (valor.trim().parse::<f64>().unwrap() * 100.0) as i64;
    let notas = [10000, 5000, 2000, 1000, 500, 200];
    let moedas = [100, 50, 25, 10, 5, 1];

    println!("NOTAS:");
    for num in &notas {
        let div = valor / num;
        println!("{} nota(s) de R$ {:.2}", div, (num / 100) as f64);
        valor = valor % num;
    }

    println!("MOEDAS:");
    for num in &moedas {
        let div = valor / num;
        println!("{} moeda(s) de R$ {:.2}", div, (*num as f64 / 100.0) as f64);
        valor = valor % num;
    }
}