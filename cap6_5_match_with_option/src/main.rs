fn main() {
    // PART 1
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // PART 2: catch-all
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // use other(catch-all)
        // o valor 9 é associado a other, e usado como argumento em move_player
    }

    // PART 3: (_) wildcard
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // use _ quando não precisar do valor
        // com o _ especificamos que não queremos o uso do valor
    }

    // PART 3.1: use () as return
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // especificamos que retornaremos vazio
        // estamos dizendo que não usaremos nenhum valor e não queremos retornar nada
    }
}

// PART 1
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        None => None,
    }
}

// PART 2
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_space: u8) {}
fn reroll() {}