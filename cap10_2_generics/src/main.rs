struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
// Método de instância implementado em Point que recebe generics X1, Y1
// mixup recebe como arg um Point com generics X2, Y2 e retorna um
// Point com generics X1 e Y2

fn main() {
    let p1 = Point{ x: 10, y: 2.5 };
    let p2 = Point{ x:"oi", y: "tim"};

    let p3 = p1.mixup(p2);
    
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}