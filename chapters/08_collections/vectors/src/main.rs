fn main() {
    // define a vector
    let v: Vec<i32> = Vec::new();

    // define a vector with macro that infers types
    let v2 = vec![1, 2, 3]; // type annotation not necesarry

    // add elements to a vector
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // reading elements of vectors
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    let second: Option<&i32> = v4.get(1);
    match second {
        Some(second) => println!("The second element is {second}"),
        None => println!("There isno second element :("),
    }

    // iterating over values in a mutable vector
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }

    // iterating over values in mutable vector and making changes to them
    let mut v6 = vec![200, 33, 89];
    for i in &mut v6 {
        println!("before {}", i);
        *i += 50;
        println!("after {}", i);
    }

    // vectors with different types through enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Super awesome string")),
    ];

    // vectors go out of scope as structs
    {
        let v7 = vec![1, 2, 3];

        // do awesome stuff with v7
    } // v7 has seized to exist, like your chances of being something at life
}
