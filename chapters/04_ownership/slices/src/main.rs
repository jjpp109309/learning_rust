fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);
    let improved_word = improved_first_word(&s[..]);
    
    let string_literal = "bye world!";
    let literal_word = improved_first_word(string_literal);

    // s.clear();

    println!("the first word is: {}", word);
    println!("the improved first word is: {}", improved_word);
    println!("the literal first word is: {}", literal_word);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if  item == b' ' {
//             return i
//         }
//     }
//
//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if  item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn improved_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if  item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
