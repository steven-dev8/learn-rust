fn main() {
    // Data type compound
    // In rust exist two primitive types: tuples and arrays
    // These types store various kinds of values

    // Tuples
    let tup: (i64, i8, f32) = (2000, -128, 5.5);

    let (x, y, z) = tup;

    println!("The value of z is: {z}");

    let one = tup.0;
    let two = tup.1;
    let three = tup.2;

    println!("{one}, {two}, {three}");

    // Arrays: store only a type element and has fixed length

    let array = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June","July",
    "August", "September", "October", "November", "December"];

    println!("day: {}, months: {}", array[0], months[11]);

    // let example: [T; L]
    let example: [i8; 3] = [1, 2, 3];
    let example2 = [3;10];

}
