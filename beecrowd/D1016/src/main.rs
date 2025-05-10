use std::io;

fn main() {
    let mut distance = String::new();
    
    io::stdin().read_line(&mut distance).unwrap();

    let mut distance: f64 = distance.trim().parse().unwrap();
                        
    distance = distance * 60.0 / 30.0;

    println!("{} minutos", distance);
}