fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1]; 
    // isso daria panic, pois não é possivel fatias partes de bytes

    println!("{s}");

    for b in hello.chars() { // .chars cria uma array de char da string
        println!("{b}")
    }

    for b in hello.bytes() { // .bytes cria uma array de bytes da string
        println!("{b}") 
    }

    // MÉTODOS

    let s1 = String::new(); // cria uma String vázia
    let s1 = String::from("Hello"); // inicializa uma String com valor
    
    let s = "hello";
    let s2 = s.to_string(); 
    // transforma elementos que possuem a trait Display 
    // implementada em String (strs, ints, char, bool...)

    let s3 = s2.clone(); // faz um clone sem perder o conteúdo de s2
    let s3 = s3.into_bytes(); // 

    // MODIFICAÇÕES

    let mut string = String::from("Teste");
    string.push('1'); // adiciona um char no final da String
    string.push_str("Teste"); // adiciona uma str no final da String
    string.insert(0, 'N'); // adiciona um char em um idx
    string.insert_str(0, "Teste"); // adiciona uma str em um idx
    string.remove(5); // remove um char do idx
    string.replace("Teste", "teste"); // troca uma string pela outra
    string.truncate(5); // corta a string até um idx
    string.clear(); // esvazia a string

    let mut nome = String::from("Sombrio");
    nome.replace_range(1..4, "tring"); // vira "Stringo"


    // LEITURA
    let len = string.len(); // retorna o tamanho da str
    let booleano = string.is_empty(); // verifica se está vazia
    let booleano = string.contains("teste"); // verifica se contém substring
    
    // BUSCA
    // starts_with(&str) / ends_with(&str) // se começa ou termina com uma substring
    // find(&str) / rfind(&str) // o primeiro ou último indice da substring

    // Transformação

    let teste = String::from("Teste");
    teste.to_uppercase(); // retorna uma lista com todas letras em maiúscula 
    teste.to_lowercase(); // letras minúsculas
    teste.trim(); // remove espaços. Variações: trim.start(), trim.end()
    teste.lines(); // separa por linhas

    let texto = String::from("Espada Sombria");
    let parte = texto.get(0..6); // Some("Espada")
    let erro  = texto.get(0..99); // None (não panica!)

    // COMPARAÇÃO
    let equal = texto.eq("Espada Sombria"); // compara com a string
}
