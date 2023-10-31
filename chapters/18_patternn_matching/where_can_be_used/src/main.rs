fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    // if let pattern matching
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Usingn purple as the background color");
        } else {
            println!("Usingn orange as the background color");
        }
    } else {
            println!("Usingn blue as the background color");
    }
    
    // while let conditional loops
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops
    let v = vec!['a', 'b', 'c'];
    
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index)
    }

    // let pattern matching
    let (x, y, z) = (1, 2, 3);

    // function patternns
    let point = (3, 5);
    print_coordinates(&point);
    
}
