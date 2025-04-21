// src/lib.rs

// RAIZ - Pai
fn delivery_order() {}

// Módulo crate::back_of_house - Filho de Raiz
mod back_of_house { 
    fn fix_incorrect_order() {
        cook_order();
        super::delivery_order(); // super acessa o módulo pai
    }

    fn cook_order() {}
}


// pub em struct
mod struct_fruit {
    // Struct
    pub struct Fruit {
        pub name: String,
        weight: u32
    }
    // Na struct você decide qual atributos devem ser pub manualmente

    impl Fruit {
        pub fn create_fruit(name_f: &str) -> Fruit {
            return Fruit {
                name: name_f.to_string(),
                weight: 1000
            }
        }
    }
}

fn register_fruit(name: &str) {
    let fruit = struct_fruit::Fruit::create_fruit(name);
    println!("{}", fruit.name);
    // println!("{}, fruit.weigth"): Erro: fruit.weight is private
}


// pub em Enums
mod enum_fruit {
    // Enum
    pub enum FruitTypes {
        Apple,
        Orange,
    }
    // No enum se ele for pub, automaticamente todas as variantes são públicas
}

fn add_fruit() {
    let type1 = enum_fruit::FruitTypes::Apple;
    let type2 = enum_fruit::FruitTypes::Orange;
}