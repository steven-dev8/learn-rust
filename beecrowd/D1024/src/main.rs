use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut inputs = stdin.lock().lines();

    let n: usize = inputs.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let linhas = inputs.next().unwrap().unwrap();
        println!("{}", criptografar(&linhas));
    }
}

fn criptografar(texto: &str) -> String {
    let primeira: Vec<char> = texto.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            (c as u8 + 3) as char
        } else {
            c
        }
        }).collect();

    let mut segunda: Vec<char> = primeira.into_iter().rev().collect();

    let meio = segunda.len() / 2;
    for c in &mut segunda[meio..] {
        *c = (*c as u8 - 1) as char;
    }

    segunda.into_iter().collect()
}
