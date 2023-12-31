struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

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

    // passing inputs that have different lifetimes: it compiles!
    let string3 = String::from("abcd");
    {
        let string4 = "xyz";
        let result = longest(string3.as_str(), string4);
        println!("The longests string is {}", result);
    }

    // passing inputs that have different lifetimes: no compilation :(
    // let string5 = String::from("long string is long");
    // let result;
    // {
    //     let string6 = String::from("xyz");
    //     result = longest(string5.as_str(), string6.as_str());
    // }
    // println!("The longests string is {}", result);


    // lifetime annotations in structs (structs that hold references)
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// function with lifetime annotation syntax
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// putting it all together: generic types, lifetimes, traits
use std::fmt::Display;

fn longest_with_an_annouoncement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
