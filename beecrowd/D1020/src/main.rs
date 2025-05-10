use std::io;

fn main() {
    let mut days_age = String::new();

    io::stdin().read_line(&mut days_age).unwrap();

    let mut days_age = days_age.trim().parse::<u16>().unwrap();
    
    let years = days_age / 365;

    days_age %= 365;
    let months = days_age / 30;
    let days = days_age % 30;

    println!("{} ano(s)", years);
    println!("{} mes(es)", months);
    println!("{} dia(s)", days);
}
