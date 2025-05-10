fn add_two(a: usize) -> usize {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello, {name}")
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Todos os módulos externos estarão disponíveis aqui

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
        // Equivalente ao assert!(result == 4);
    }

    #[test]
    fn greeting_no_contains_name() {
        let result = greeting("Steven");
        assert_ne!(result, "Livia");
        // Equivalente ao assert!(result != "Livia");
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Rosane");
        assert!(
            result.contains("Misato"),
            "Greeting did not contain name, value was `{result}`"
        ); // O segundo argumento serva para passar uma mensagem personalizada do erro
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Assert (bool): Verifica booleanos, quando for true o teste passa, mas se for
        // false o teste reprova
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }

}
