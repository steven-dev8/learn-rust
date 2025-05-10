pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

fn prints_and_return10(n: i32) -> i32 {
    println!("This number is {n}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    // expected guarda uma string que deve estar contida na mensagem de panic; caso não esteja,
    // o teste reprova. Então temos aqui duas verificações: a primeira é se a função realmente dá panic,
    // e a segunda é se a string dentro de `expected` aparece dentro da mensagem do panic.
    fn greater_than_100() {
        Guess::new(200);
        // Guess::new(-1) daria erro
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_return10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_return10(4);
        assert_eq!(value, 4);
    }
    // para mostrar os prints dentro de um teste se utiliza "cargo test -- --show-output"
    // para executar um subconjunto de testes, utilizamos cargo test (nome(s) do(s) teste(s))
    // Exemplo: cargo test this_test_will_pass
    // Exemplo: cargo test this     Isso executara todos os testes com this no nome
    // #[ignore] para ignorar testes, para executar testes ignorados "cargo teste -- --ignored"
    // cargo test -- --include-ignored, para executar tanto testes ignorados quanto os permitidos
}

