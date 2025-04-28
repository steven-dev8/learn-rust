// 'c é o lifetime da referência em `part`, garantindo que ela seja válida enquanto o parâmetro existir.
struct ImportantExcerpt<'c> {
    part: &'c str,
}

impl<'a> ImportantExcerpt<'a> {
    // Método que retorna uma referência para `part`, com o mesmo lifetime de `self`.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part  // O lifetime de `part` é o mesmo de `self`, ou seja, `'a`.
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();  // lifetime de `first_sentence` é vinculado ao `novel`.

    // `first_sentence` é passado como referência, com o mesmo lifetime de `novel`.
    let i = ImportantExcerpt {
        part: first_sentence,  
    };

    // 'static é o lifetime de dados que vivem durante toda a execução do programa (ex: literais de string).
    let s: &'static str = "I have a static lifetime.";
}
