enum Lista {
    Int(i64),
    Float(f64),
    Text(String),
}

fn main() {
    let mut lista: Vec<Lista> = Vec::new();
    lista.push(Lista::Int(50));
    lista.push(Lista::Text(String::from("blue")));
    lista.push(Lista::Float(30.5));
    // Dessa forma consigo guardar vários tipos diferentes no meu Vector

    // VETORES (Array dinâmica)
    // MÉTODOS DE INICIALIZAÇÃO

    // Declarando e inicializando um Vetor
    let vetor: Vec<i32> = Vec::new();

    // Inicializar um vetor com elementos usando o macro vec!
    let mut vetor = vec![1, 2, 3];

    // Criar um vetor com capacidade pré-alocada
    let mut vetor = Vec::with_capacity(10);

    // MÉTODOS DE ADIÇÃO

    // Adiciona 4 como último elemento do vetor
    vetor.push(4);

    // Adicionar uma lista de valores
    vetor.extend([5, 6, 7]);

    // Inseri um valor em uma posição
    vetor.insert(0, -1); // Adiciona -1 na posição 0

    // MÉTODOS DE REMOÇÃO

    // Remover e retornar o último elemento
    vetor.pop();

    // Remover o elemento de uma posição
    vetor.remove(0); // Remove o elemento da posição 0

    // MÉTODOS DE ACESSO

    // Acesso via indexação direta
    let first = vetor[0];

    // Acesso via index e retorna um Option, usado em match
    let second = vetor.get(1);

    match second {
        Some(second) => println!("Tem um valor e ele é o {second}"),
        None => println!("Não há um elemento nesse index"),
    };

    // Iteração

    // itera sobre referências imutáveis
    for x in vetor.iter() { // equivalente ao &vetor
        println!("{x}")
    }

    // itera sobre referências mútaveis
    for x in vetor.iter_mut() { // equivalente ao &mut vetor
        *x = *x * 10;
    }

    // Métodos úteis

    // ler o tamanho do vetor e retorna o seu tamanho
    println!("Length is {}", vetor.len());

    // verifica se o vetor está vazio
    if vetor.is_empty() {
        println!("Vector is empty")
    };

    // verifica se o vetor tem um valor
    if vetor.contains(&5) {
        println!("Vector has value 5")
    };

    // ordenar um vetor
    vetor.sort();

    // reverter a ordem
    vetor.reverse();

    // remove elementos duplicados consecutivos
    vetor.dedup();

    // limpar um vetor
    vetor.clear();
}
