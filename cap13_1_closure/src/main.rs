#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        // unwrap tenta extrair o valor, caso contrário, ele executa uma closure
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_blue = 0;
        let mut num_red = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    let inventory = Inventory { shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red] };

    let client1 = Some(ShirtColor::Red);
    let shirt1 = inventory.giveaway(client1);
    println!(
        "The user with preference {:?} gets {:?}",
        client1.unwrap(), shirt1
    );

    let client2 = None;
    let shirt2 = inventory.giveaway(client2);
    println!(
        "The user with preference {:?} gets {:?}",
        client2, shirt2
    );

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }  // Função
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // Closure explicita
    let add_one_v3 = |x|             { x + 1 }; // Closure sem explicitar tipos
    let add_one_v4 = |x|               x + 1  ; // Closure sem chaves de retorno

    add_one_v3(5);
    add_one_v4(5);

    let example = |x| x;

    example(String::from("Hello"));
    // example(5); Error
    // O tipo de um Closure é difinido na sua primeiro ocorrência de chamada
    // nesse ponto o tipo inferido de example é apenas String
}   
