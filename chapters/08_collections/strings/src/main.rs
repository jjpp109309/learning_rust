fn main() {
    // new string
    let mut s = String::new();

    // instanciate a string (2 methods)
    let data = "initial contents";
    let s = data.to_string();

    let s = "initial string".to_string();
    
    // append string
    let mut s2 = String::from("foo");
    s2.push_str("bar");
    println!("{}", s2);

    // push_str uses slice to avoid taking ownership
    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    println!("{}", s4); // no problem since we didn't take ownership

    // we can also push chars with push
    let mut s5 = String::from("lo");
    s5.push('l');
    println!("s5 is {}", s5);

    // combining two strings with + operator
    let s1 = String::from("Hello");
    let s2 = String::from("World");

    let s3 = s1 + &s2; // s1 can no longer be used
    println!("{}", s3);

    // concatenating muliple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3; // method 1
    // println!("concatenation method 1: {}", s)

    let s = format!("{s1}+{s2}+{s3}"); // method 2 for more complicated formats
    println!("concatenation method 2: {}", s);
    
    // there is no standard way of indexing in rust
    // we can iterate over strings with .char() method
    for c in "cosacosa".chars() {
        println!("{}", c);
    }


}
