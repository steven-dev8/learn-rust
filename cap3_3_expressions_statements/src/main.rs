fn main() {
    // Expression: anything that returns a value.
    // Statement: anything that does not return a value.

    // These are expressions
    let x = (1 + 1);
    let y = {
        let x = 6;
        x + 1
    }; // It will return 7 to y

    // This is a statement
    let x = 2;
}
