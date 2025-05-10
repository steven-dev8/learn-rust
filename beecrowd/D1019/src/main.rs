use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();

    let mut total_seconds = input.trim().parse::<i64>().unwrap();
    let mut time_units = [3600, 60, 1];

    for unit in &mut time_units {
        let value = total_seconds / *unit;
        total_seconds %= *unit;

        *unit = value;
    }

    println!("{}:{}:{}", time_units[0], time_units[1], time_units[2]);
}