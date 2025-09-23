#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    // Add other states as needed
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter! {:?}", state);
            25
        }
    }
}



fn main() {
    let coin = Coin::Quarter(USState::Alabama);
    let cents = value_in_cents(coin);
    println!("The coin is worth {} cents", cents);
}
