use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { // Self se refêrencia ao proprio tipo, que no caso é Pair<T>
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
// Vou implementar um método para um generic que implemente as trait
// Display e PartialOrd
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

fn main() {
    let point1 = Pair::new(10, 20);
    point1.cmp_display();
    // 10 e 20 são do tipo i32 que implementa Display e PartialOrd

    let point2 = Pair::new([10], [20]);
    // point2.cmp_display(); isso exibe um erro
}
