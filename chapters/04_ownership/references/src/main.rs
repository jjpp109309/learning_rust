fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of the string \"{s1}\" is {len}");

    // we cannot alter a borrowed value within a function
    // let s = String::from("hello");
    // change(&s);

    // mutable reference
    let mut s = String::from("hello");
    change(&mut s);

    // cannnot borrow more than once the same variable
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    println!("{}", r1);

    // we can borrwo more than once in different spots    
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here

    let r2 = &mut s;
    println!("{}", r2);

    // cannot combine mutable and immutable refernces
    // let mut s = String::from("hello");
    //
    // let s1 = &s; // no problem
    // let s2 = &s; // no problem
    // let s3 = &mut s; // PROBLEM
    //
    // println!("{}, {}, and {}", s1, s2, s3);

    // multiple mutable references can occurr and mutable reference can
    // be introduced only after the former references have been used

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    println!("{}, {}", r1, r2); // r1 and r2 are not valid after this point

    let r3 = &mut s;
    println!("{}", r3);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// NOT THIS
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}



