fn main() {
    let mut v = vec![1, 2, 3, 4 ,5];
    let first = v[0];
    

    v.push(6);
    // let first = &v[0], a linha de cima daria erro (borrow checker)

    for i in &v {
        println!("{i}")
    }

    for i in &mut v { // cria referências MUTÁVEL e atribui a i
        *i += 50; // * desreferência i
        println!("{i}"); 
    }
}
