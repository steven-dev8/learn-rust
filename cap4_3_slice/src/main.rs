fn main() {
    // Part1
    let mut s1 = String::from("Hello Amigos");
    let word = first_word1(&s1); // retorna um usize (posição do primeiro espaço)

    s1.clear(); // Limpa a String, agora s1 = ""

    // println!("{word}"); isso da certo, pois "word" é só um número
    // MAS se tentássemos usar esse índice depois para acessar a String (como s1[word]),
    // poderíamos causar bugs ou panics, dependendo do conteúdo da String


    // Part2
    let mut s2 = String::from("Rust"); 
    let word = first_word2(&s2); // retorna uma slice

    s2.clear();

    // println!("{word}"); Error: você está tentando acessar
    // um conteúdo que já foi apagado por clear()

    // E se eu redeclarasse ou adicionasse uma nova string em s2?
    // Ainda daria erro, pois "word" está apontando para os dados antigos
    // de "s2", e o Rust não permite isso por segurança

}

// Não faz uma ligação com o dado da String
fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes(); // Converte em uma matriz de bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i - 1;
        }
    }
    s.len()
}

// Faz ligação via refêrencia com a String
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // Retorna a referência do ínicio ao index i
            // tip: poderia ser escrito &s[..i];
        }
    }
    return &s[..] // Retorna a referência do ínicio ao final
} 