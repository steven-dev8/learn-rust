#[derive(Debug)]
enum Recursion<'a> {
    Item(i32, Rc<RefCell<Option<&'a Recursion<'a>>>>),
}

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut value: Option<Recursion> = None;

    let a = Recursion::Item(10, Rc::new(RefCell::new(&value)));
    let b = Recursion::Item(20, Rc::new(RefCell::new(Some(&a))));

    println!("{:?}", a);
}
