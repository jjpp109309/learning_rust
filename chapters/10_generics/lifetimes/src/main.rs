fn main() {
    // won't compile
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }

    let x = 5;
    let r = &x;

    println!("r: {}", r);

    // generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longests string is {}", result);
}

// function with lifetime annotation syntax
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
