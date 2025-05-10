struct Guess {
    value: i32
}

impl Guess {
    fn guessing_number(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Choose a value beetwen 1 and 100");
        }
        Guess {value}
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let numbers = vec![5, 2, 1, 45, 23, 51, 32, 53, 32, 57];

    let mut largest = &numbers[0];

    for num in &numbers {
        if num > largest {
            largest = num
        }
    }

    println!("{largest}");
}
