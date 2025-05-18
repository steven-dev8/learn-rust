#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(10));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(11)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(12)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    // Desreferencio Rc (que implementa deref) e consigo mutar o valor de dentro
    // de RefCell() com um método da API chamada borrow_mut() += 10;

    println!("a after = {a:?}");    
    println!("b after = {b:?}");
    println!("c after = {c:?}");    
}
