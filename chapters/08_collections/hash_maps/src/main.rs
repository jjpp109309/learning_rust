fn main() {
    // creating a hash map
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // all keys and values must share same type respectively

    // extract values from scores
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("score of {} is {}", team_name, score);

    // iterate over hash map
    for (key, value) in &scores {
        println!("Team {} score {}", key, value);
    }

    // values that can be coppied are coppied into hashmap, else are moved
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // from this point field and value can no longer be used 

    // overwritting values
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 25);

    println!("{:?}", scores);

    // adding only new values without overwritting existing onoes

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);

    println!("{:?}", scores);

    // using value of hashmap and upating it
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
