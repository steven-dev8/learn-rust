use std::io;

fn main() {
    let mut ponto1 = String::new();
    let mut ponto2 = String::new();

    io::stdin().read_line(&mut ponto1).unwrap();
    io::stdin().read_line(&mut ponto2).unwrap();    

    let ponto1: Vec<f64> = ponto1.trim().split_whitespace()
                                .map(|x| x.parse::<f64>().unwrap())
                                .collect();
    let ponto2: Vec<f64> = ponto2.trim().split_whitespace()
                                .map(|x| x.parse::<f64>().unwrap())
                                .collect();
                                
    let (x1, y1) = (ponto1[0], ponto1[1]);
    let (x2, y2) = (ponto2[0], ponto2[1]);

    let distancia = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();

    println!("{:.4}", distancia);
}