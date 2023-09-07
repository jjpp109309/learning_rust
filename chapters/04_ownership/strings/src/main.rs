fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    // data interacting with move
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); not allowed cause s1 is invalid after preivous line
    println!("{}, world!", s2); // s2 is valid

    // deep copies using clone method
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // ownership in function calling
    let s = String::from("hello");
    takes_ownership(s); // after this line we can no longer use s
    
    let x = 5;
    makes_copy(x); // we can still use x after this 'cause its value get's copied not moved
    println!("{x}");

    // return values from functions also get moved
    let s1 = gives_ownership();
    println!("{s1}");

    // we can get the moved value back by returning a tuple...
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    println!("the length of {} is {}", s2, len);
    // ... but this is just too ceremonious and unnecessary when we learn about references

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    
    (s, length)
}
