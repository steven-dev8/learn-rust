use std::io;

fn main() {
    let mut val = String::new();

    io::stdin().read_line(&mut val).unwrap();

    let val: Vec<i32> = val.trim().split_whitespace()
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect();
    
    let cmp_a_b = (val[0] + val[1] + (val[0] - val[1]).abs()) / 2;
    let result = (cmp_a_b + val[2] + (cmp_a_b - val[2]).abs()) / 2;

    println!("{} eh o maior", result);
}
