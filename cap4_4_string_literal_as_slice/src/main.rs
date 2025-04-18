fn main() {
    let s1 = "Steven Araújo";
    let s2 = String::from("Livia Araújo da Costa");

    let word = first_word(&s1); // Literal string
    println!("{word}");

    let word = first_word(&s2); // String
    println!("{word}");

    let word = first_word(&s2[6..]); // Slice string
    println!("{word}")
}

// First word aceita tanto uma string literal, String ou uma slice string
fn first_word(s: &str) -> &str { 
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}
