use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
    // o Rust internamente executou *(y.deref());

    // Coerção de tipos
    let st1 = MyBox::new(String::from("Olá 1"));
    hello(&st1);

    let st2 = MyBox::new("Olá 2");
    hello(&st2);
}

fn hello(x: &str) {
    println!("Hello {}", x);
}
