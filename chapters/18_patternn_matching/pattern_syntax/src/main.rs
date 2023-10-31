struct Point {
    x: i32,
    y: i32,
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three "),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
    
    // multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructurin structs
    let p = Point { x: 0, y: 7};

    let Point { x: a, y: b} = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point {x, y: 0} => println!("On the x axis"),
        Point {x: 0, y} => println!("On the y axis"),
        Point {x, y } => {
            println!("on neighter axis: ({}, {})", x, y)
        },
    }

    // destructing enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("the quit variant has no data :(");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to rgb({},{},{})", r, g, b);
        }
    }

    // nested enums
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to rgb({},{},{})", r, g, b);
        },
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hsv({},{},{})", h, s, v);
        },
        _ => ()
    }
}
