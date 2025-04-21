fn main() {
    // VERSION 1.0
    let value: Option<u8> = None;
    match value {
        Some(val) => println!("{val}"),
        _ => (),
    };
    
    // VERSION 2.0 - Sugar Syntax
    if let Some(val) = value {
        println!("{val}")
    }

    // VERSION 3.0 - Else
    if let Some(val) = value {
        println!("{val}")
    } else {
        println!("Nada")
    }
}
