use std::io;

fn main() {
    convert_temp();
}

fn convert_temp() {
    println!("Temperature converter between Fahrenheit and Celsius.");

    loop {
        println!("Input F to Fahrenheit or C to Celsius: ");
        let mut choose = String::new();

        io::stdin()
        .read_line(&mut choose)
        .expect("Error: could not read line.");
        
        let choose = choose.trim();
        let mut number = String::new();

        if choose == "F" || choose == "C" {
            println!("Input the temperature: ");
            io::stdin()
            .read_line(&mut number)
            .expect("Error: could not read line.");

            let number: f32 = match number.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("That's not a number.");
                    continue;
                }
            };

            if choose == "F" {
                println!("{number} °F = {:.2} °C", (number - 32.0) * 5.0/9.0);
                break;
            } else {
                println!("{number} °C = {:.2} °F", (number * 1.8) + 32.0);
                break;
            }
        } else {
            println!("Type F or C, stupid !!!");
        }
    }
}