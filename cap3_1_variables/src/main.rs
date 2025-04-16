fn main() {
    // Shadowing
    let x = 5;

    let x = x + 1;
    println!("The value of X is {x}");

    {
        let x = x * x;
        println!("The value of X in scope is {x}");
    }

    println!("The value of X in out scope is {x}\n");

    let space = "     ";
    let space = space.len();
    println!("{}", space); // output: 5

    // Error: mut doesn't change type
    // let mut space_test = "     ";
    //space = space_test.len();    
}