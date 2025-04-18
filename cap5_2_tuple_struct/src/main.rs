struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual; // Struct unitária

fn main() {
    // PART 1
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // mesmo black e origin tendo a mesma estrutura e aceitando os mesmo valores
    // ambas são de tipos diferentes. Ex: uma função que recebe Color não pode receber
    // Point como argumento

    println!("{}", black.0); // Acessa o primeiro valor da tuple struct

    // DESESTRUTURAR TUPLE STRUCT
    let Point(x, y, z) = origin; 
    // Tipo(variaveis que vão ser atribuidas) = instância do tipo

    // Diferente de como as tuplas normais são desestruturadas
    let tuple = (1, 2, 3);
    let (x, y, z) = tuple;

    // PART 2: unit struct
    let subject = AlwaysEqual;
    // Uma struct unitária é uma struct sem dados
}
