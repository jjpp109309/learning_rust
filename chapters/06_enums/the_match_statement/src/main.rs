#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // and the rest of the countries 
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let my_coin = Coin::Penny;
    let my_coin_value = value_in_cents(my_coin);

    let my_other_coin = Coin::Quarter(UsState::Alaska);
    let my_other_coin_value = value_in_cents(my_other_coin);

    println!("My coin value is: ${my_coin_value}");
    println!("My other coin value is: ${my_other_coin_value}");

    
}

fn value_in_cents(coin: Coin) -> u8 {
    // with if expressions we would have to evaluate conditions to bools,
    // with match we can equate to any type!!!
    match coin {
        // we can have multiple lines per arm using braces syntax
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }


}
