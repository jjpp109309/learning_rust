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

    // matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // matches must exhaust all the possible values of the enum

    // matches with default values for remaining patterns in enum
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ => reroll() // in case we want to reroll until landing on 3 or 7
        // _ => () // do nothing if we land in the last arm
    }

}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }

}
