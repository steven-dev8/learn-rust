use std::io;

fn main() {
    static GASTO_AUTO: u8 = 12;

    let mut temp_gasto = String::new();
    let mut velocidade_media = String::new();

    io::stdin().read_line(&mut temp_gasto).unwrap();
    io::stdin().read_line(&mut velocidade_media).unwrap();    

    let temp_gasto = temp_gasto.trim().parse::<f64>().unwrap();
    let velocidade_media = velocidade_media.trim().parse::<f64>().unwrap();

    let distancia_percorrida = temp_gasto * velocidade_media;
    let gasto_total = distancia_percorrida / GASTO_AUTO as f64;

    println!("{:.3}", gasto_total);
}
