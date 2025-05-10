use std::io;

fn main() {
    let mut money = String::new();
    
    io::stdin().read_line(&mut money).unwrap();
    
    let cedulas = [100, 50, 20, 10, 5, 2, 1];
    let mut money = money.trim().parse::<i64>().unwrap();

    println!("{}", money);

    for value in &cedulas {
        let notas = money / value;
        money = money % value;
        println!("{} nota(s) de R$ {},00", notas, value);
    }
}
