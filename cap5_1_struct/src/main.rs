struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // PART 1: tipos de instâncias 
    // Instanciar manualmente
    let mut user1 = User {
        active: true,
        username: String::from("steven3d"),
        email: String::from("steven@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("steven3d@gmail.com"); // Acessando valores com .

    // Instanciar de forma autómatica com fn
    let user2 = build_user(String::from("teste1d"), String::from("teste@gmail.com"));

    // PART 2: instâncias de outras instâncias
    // Instânciar manualmente, sem syntax de update
    let user3 = User {
        active: user2.active,
        username: user2.username,
        email: String::from("outro@gmail.com"),
        sign_in_count: user2.sign_in_count,
    };

    // Com syntax de update
    let user4 = User {
        username: String::from("outro3dd"),
        ..user3 // todos os outros atributos de user3, vão ser passados
        // automáticamente para user4
    };

    println!("Username: {}", user4.username);
    println!("Email: {}", user4.email);
    println!("Active: {}", user4.active);
    println!("{}", user2.sign_in_count);
    // Tentar acessar algumas própriedade de user2 (username) ou user3 (email)
    // causará erros de borrow, pois a posse atual desses atributos é de user4    
}

// Sem field init
fn build_user(name: String, email: String) -> User {
    User {
        active: true,
        username: name,
        email: email,
        sign_in_count: 1
    }
}

// Com field init
fn build_user_field_init(username: String, email: String) -> User {
    User {
        active: true,
        username, // Não é necessário atribuir o username e name aqui
        email, // pois os parâmetros já tem o mesmo nome dos atributos
        sign_in_count: 1
    }
}