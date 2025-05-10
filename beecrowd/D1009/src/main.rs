use std::io;

fn main() {
    let mut name = String::new();
    let mut salary = String::new();
    let mut amount = String::new();

    io::stdin().read_line(&mut name).unwrap();    
    io::stdin().read_line(&mut salary).unwrap();
    io::stdin().read_line(&mut amount).unwrap();

    let (salary, amount) = (salary.trim().parse::<f64>().unwrap(),
                            amount.trim().parse::<f64>().unwrap());
    
    println!("TOTAL = R$ {:.2}", salary + amount * 0.15);
}
