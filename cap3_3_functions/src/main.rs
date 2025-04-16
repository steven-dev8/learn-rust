fn main() {
    println!("Hello, world!");

    another_function1();

    another_function2(5);

    print_labeled_measurement(5, 'S');
}

fn another_function1() {
    println!("Another function.");
}

fn another_function2(x: i8) {
    println!("The value of x is: {x}");
    println!("The double of x is: {}", x * 2);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}