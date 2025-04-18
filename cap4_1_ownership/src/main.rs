fn main() {
    let x = 3;
    let y = x; // int and float have trait copy
    // then y copied x

    let s1 = String::from("Olá");
    let s2 = s1; // String has trait clone
    // s2 took ownership of s1, s1 no longer exists

    let s3 = s2.clone(); // clone copies the value of s2
    // without getting the property

    println!("y value: {} x value: {}", y, x);
    println!("s3 value: {}, s2 value: {}", s3, s2);
    // println!("{}", s1); ERROR: s3 no longer exist
}
