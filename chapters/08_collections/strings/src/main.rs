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

}
