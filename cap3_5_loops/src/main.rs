// RUST
fn main() {
    let mut c = 0;

    loop {
        c += 1;
        println!("{c}");
        
        if c == 3 {break};
    }
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
        // This raise error: else {break "{counter}"};
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;
    }
    println!("End count = {count}");
}
