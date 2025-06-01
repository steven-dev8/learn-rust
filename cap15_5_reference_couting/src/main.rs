enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
// Rc<T> é um ponteiro inteligente que gerência o número de referências de forma automática
// quando ele percebe que não há referências apontando para um valor na heap, ele libera esse valor

fn main() {
    let a = Rc::new(Cons(10, Rc::new(Cons(11, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
    let c = Cons(2, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // c, que também era dono de a, acaba sendo dropado no final do bloco
    // o número de referências diminui
    
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
