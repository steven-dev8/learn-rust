fn main() {
    let arr = [10, 20, 30, 40, 50];
    let mut idx = 0;

    while idx < 5 {
        println!("the value is: {}", arr[idx]);

        idx += 1;
    }

    for element in arr {
        println!("the value is: {element}");
    }

    for number in (1..4) {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}