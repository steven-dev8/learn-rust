fn main() {
    let list = vec![1, 2, 5];

    let closure = || println!("The elements of list is {list:?}");
    // a referência de list na closure é imutável
    
    println!("after: {list:?}");
    closure();
    println!("before: {list:?}");

    let mut list = vec![5, 1, 2];

    let mut closure = || list.push(10);
    // a referência de list e closure são mutáveis

    // println!("after: {list:?}"); 
    // Error, não é possivel usar uma referência mutável ao mesmo tempo que uma imutável 
    closure(); // Fim da referência mutável
    println!("before: {list:?}"); // Pega uma referência imutável de list


    // Para mover a propriedade se utiliza o move
    let list = vec![3,3,3];

    let closure = move || println!("{list:?}");

    closure();
    // println!("{list:?}") Error, o valor foi movido
}