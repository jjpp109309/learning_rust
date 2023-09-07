fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("contidion was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3, or 4");
    }

    let condition: bool = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; all branches must have the same type

    println!("The value of number is: {number}");

    // infinite loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }

        println!("Condition not met :(");
    };
    
    println!("The result is {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            } 
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loops
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // run for loop n times
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
