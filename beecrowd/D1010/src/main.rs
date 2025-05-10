use std::io;

fn main() {
    let (mut l1, mut l2) = (String::new() ,String::new());

    io::stdin().read_line(&mut l1).unwrap();
    io::stdin().read_line(&mut l2).unwrap();
    
    let mut l1 = l1.trim().split_whitespace();
    let mut l2 = l2.trim().split_whitespace();

    l1.next();
    l2.next();

    let value1 = l1.next().unwrap().parse::<f64>().unwrap() *
                 l1.next().unwrap().parse::<f64>().unwrap();
    
    let value2 = l2.next().unwrap().parse::<f64>().unwrap() *
                 l2.next().unwrap().parse::<f64>().unwrap();
    
    println!("VALOR A PAGAR: R$ {:.2}", value1 + value2);
}
