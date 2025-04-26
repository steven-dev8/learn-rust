use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fl) => fl,
                Err(err) => panic!("Problem creating the file: {err:?}"),
            },
            other_error => panic!("Problem opening the file: {other_error:?}")
        }
    };

    // let file = File::open("test.txt").unwrap();
    // unwrap() retorna o valor caso ele seja Ok or Some, caso contrário 
    // chamará um panic!

    let file = File::open("test.txt")
        .expect("teste.txt should be inluded in this project");
    // expect() retorna um panic caso o valor seja um Err or None
    // dentro você pode customizar a mensagem para o panic!
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => return Ok(username),
        Err(e) => return Err(e),
    };
}

fn read_username_from_fileV2() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?; 
    // o "?" retorna um Err caso o result retorne Err, é um match simplificado
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
    // a linha 50 vai tentar ler o arquivo e retornar um string para username,
    // se a operação der certo, é retornado um Ok(username) na ln51, caso falhe
    // é retornado um erro, por isso usamos o ?
    // "?" só pode ser usado em funções que retorna Option ou Result
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // .lines() retorna um iterador de linhas do text
}
