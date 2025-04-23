use std::collections::HashMap; // importa um hashmap, pois ele não faz parte do prelúdio

fn main() {
    let mut scores = HashMap::new(); // cria um hashmap vázio

    scores.insert(String::from("Blue"), 10); // insert insere um par-chave no hashmap.
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Yellow");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // .get(&String) tenta pegar um valor associado aquela chave, retorna um Option<&V>.
    // .copied() desreferência o Option<&V> para Option<V>.
    // .unwrap_or() caso não ache e o valor seja None, define um valor padrão.

    println!("{score}");

    for (key, value) in &scores { // iterar sobre hashmap.
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 50); // sobrescreve o valor atual de Blue.
    scores.entry(String::from("Blue")).or_insert(100); 
    // se não houver valor na chave blue, adiciona 100.
    // .entry retorna um Entry, que identifica se há um valor ou não

    let mut hash = HashMap::new();
    let text = "Hello World wonderful world";

    for word in text.split_whitespace() {
        // .entry(word) acessa a entrada da chave 'word' na HashMap (existente ou não)
        // .or_insert(0) insere 0 se não existir e retorna uma &mut i32 (referência mutável)
        let count = hash.entry(word).or_insert(0); 
        *count += 1;
    }

}
