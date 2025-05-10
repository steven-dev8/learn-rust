use std::io;

fn main() {
    let mut values = String::new();

    io::stdin().read_line(&mut values).unwrap();

    let values: Vec<i64> = values.trim().split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap()).collect();
    
    let (a, b, c, d) = (values[0], values[1], values[2], values[3]);

    if b > c && d > a && (c + d) > (a + b) && c > 0 && d > 0 && a % 2 == 0 {
        println!("Valores aceitos");
    } else {
        println!("Valores nao aceitos");
    }
}
