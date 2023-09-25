enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    
}

fn value_in_cents(coin: Coin) -> u8 {
    // with if expressions we would have to evaluate conditions to bools,
    // with match we can equate to any type!!!
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
