fn main() {
    let mut s = String::new(); // cria uma String

    let mut data = "Silva"; // Isso é uma &str
    let s1 = data.to_string(); 
    // to_string converte qualquer dado para String, suportado em todos os tipos que
    // implementam o Display (char, bool, strs, números...)

    // UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");

    // MÉTODOS COM STRING

    // adiciona elemento na String
    let mut hello = String::from("Hello");
    let world = " World";
    hello.push_str(world);
    println!("{hello}, {}", world);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // concatenação equivalentes
    let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}")
}
