struct Point<T> { // Point<T, U>, com mais de um generic
    x: T,
    y: T,
}

impl<T> Point<T> { // Aplicando um método para qualquer tipo em Point
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> { // Aplicando um método para um tipo específico em Point
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// With generics
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let list = vec![1, 9, 3, 5, 2];
    println!("{}", largest(&list));

    let integer = Point{x:10, y:20};
    let float = Point{x:1.1, y:1.1};
    
    println!("Value of X: {}", integer.x());
    println!("Distance from origin: {}", float.distance_from_origin());
    // integer.distance_from_origin()
    // Error: integer é um Point<i32> e o método é implementado apenas para f32
}