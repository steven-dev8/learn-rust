#[derive(Debug)] // Permite imprimir tipos
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter recebe um tipo UsState
}

fn value_in_cents(coin: &Coin) -> u8 { // recebe uma referência do tipo coin e retorna um u8
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // state é um argumento do tipo  UsState
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    // coin recebe uma instância da variante Quarter do enum Coin,
    // que por sua vez recebe como parâmetro uma instância do enum UsState.

    let result = value_in_cents(&coin);
}
