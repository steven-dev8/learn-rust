struct NumberPhone {
    number: String
}

impl Drop for NumberPhone {
    // a função de drop sempre recebe um &mut self
    fn drop(&mut self) {
        println!("Dropping number {}", self.number);
    }
}

// Todos os tipos em Rust são dropados (destruídos) automaticamente no fim do escopo
// Implementando a trait Drop, você pode customizar esse processo,
// executando código antes da destruição acontecer

fn main() {
    let n1 = NumberPhone { number: String::from("999") };
    let n2 = NumberPhone { number: String::from("1010") };

    println!("Created number here");
    drop(n1); // n1.drop() da error
    drop(n2);
    // a função drop é um destrutor, análogo a um construtor, ele vai destruir a instância
    // antes dela chegar no final do escopo
    // drop vem da std::mem::drop
    println!("Droped")
}