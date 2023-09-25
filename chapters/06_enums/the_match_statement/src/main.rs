enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let my_coin = Coin::Penny;
    let my_coin_value = value_in_cents(my_coin);

    println!("My coin value is: ${my_coin_value}");
    
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
        Coin::Quarter => 25,
    }
}
