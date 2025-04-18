fn main() {
    // part1
    let mut s1 = String::from("n_louco");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // part2
    change(&mut s1);
    println!("{s1}");

    // part3: multable references 
    let mut s2 = String::from("stevennn");
    let r1 = &mut s2;
    // let r2 = &mut s2; ERROR
    // println!("{}, {}", r1, r2) ERROR
    // solution: create a new scope before another reference

    // part4: multable references and immutable references
    let r3 = &s2; // no problem
    let r4 = &s2; // no problem
    // let r5 = &mut s2 - BIG PROBLEM 

    println!("{}, {}", r3, r4);
    // r3 and r4 will not be used after this point
    let r5 = &mut s2; // no problem
    
    // part5: define variables and return references (dangling references)
    // dangle();
    let s3 = no_dangle();

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" é louco");
}

// Example function: line 27
// fn dangle() -> &String {
// let s = String::from("hello");
// &s
// }

fn no_dangle() -> String {
    let s = String::from("antony gosta de orquestra");
    s // this return a String and not a reference
}