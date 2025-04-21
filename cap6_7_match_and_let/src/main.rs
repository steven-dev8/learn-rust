#[derive(Debug)]
enum Real {
    Centavos,
    Reais,
}

enum Coin {
    Brasil(Real),
    China,
    Alemanha
}

impl Real {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            Real::Centavos => year <= 2000,
            Real::Reais => year <= 1994,
        }
    }
}

fn main() {
    let coin = Coin::Brasil(Real::Reais);
    let year = 1995;
    let phrase = describe_coin3(&coin, year);

    if let Some(value) = phrase { // Não ta caindo aqui
        println!("{value}")
    }
    
}

fn describe_coin1(coin: &Coin, year: u16) -> Option<String> { // if let
    if let Coin::Brasil(state) = coin {
        if state.existed_in(year) {
            return Some(format!("Sua moeda {state:?} é bastante antiga"));
        } else {
            return Some(format!("Sua moeda {state:?} é recente"));
        }
    }
    None
}

fn describe_coin2(coin: &Coin, year: u16) -> Option<String> { // let if
    let state = if let Coin::Brasil(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(year) {
        return Some(format!("Sua moeda {state:?} é bastante antiga"));
    } else {
        return Some(format!("Sua moeda {state:?} é recente"));
    }
}

fn describe_coin3(coin: &Coin, year: u16) -> Option<String> { // let else
    let Coin::Brasil(state) = coin else {
        return None;
    };

    if state.existed_in(year) {
        return Some(format!("Sua moeda {state:?} é bastante antiga"));
    } else {
        return Some(format!("Sua moeda {state:?} é recente"));
    }
}