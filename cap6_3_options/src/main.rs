struct User {
    name: String,
    age: Option<u8>,
}

fn main() {
    let mut some_number = Some(5); // Identifica o tipo do option automáticamente
    let some_char = Some('A'); // Option<char>
    some_number = None;

    let user1 = User {
        name: String::from("Steven"),
        age: None,
    };

    let user2 = User {
        name: String::from("Bernardo"),
        age: Some(20)
    };

    let arr: [User; 2] = [user1, user2];

    for user in &arr { //.iter() já retorna referências dos itens
        match user.age {
            Some(age) => println!("{} têm {} anos", user.name, age),
            None => println!("{} não possui a idade cadastrada", user.name),
        };
    };
}
