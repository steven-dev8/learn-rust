struct NumberPhone {
    number: String
}

impl Drop for NumberPhone {
    fn drop(&mut self) {
        println!("Apagando o número {}", self.number);
    }
}

fn main() {
    let a = NumberPhone {
        number: String::from("53999003838")
    };

    let b = NumberPhone {
        number: String::from("43559904938")
    };

    println!("Números criados");

}
