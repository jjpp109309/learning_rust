use std::collections::HashMap;

fn main() {

    let my_vector = vec![3.0, 5.0, 6.0, 7.0];
    let average = mean(&my_vector);
    println!("The average is {}", average);

    let mut my_vector2 = vec![6, 5, 8, 0, 2];
    let med = median(&mut my_vector2);
    println!("The median is {}", med);

    let mut my_vector3 = vec![6, 5, 0, 2];
    let med2 = median(&mut my_vector3);
    println!("The median is {}", med2);
    
    let my_vector3 = vec![1, 2, 4, 4, 5];
    let mode2 = mode(&my_vector3);
    println!("The mode is {}", mode2);

    let my_word = String::from("apple");
    let my_pig_word = pig_latin(&my_word);
    println!("The pig latin word for {} is {}", my_word, my_pig_word);
}


fn mean(list: &Vec<f32>) -> f32 {
    let mut sum = 0.0;
    let count = list.len() as f32;

    for value in list {
        sum += value;
    }

    sum / count
}

fn median(list: &mut Vec<i32>) -> i32 {

    let middle_index = list.len() / 2;
    list.sort();

    list[middle_index]
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    for number in list {
        let count = counts.entry(number).or_insert(0);
        *count += 1;
    }

    let mut value = 0;

    for (number, count) in counts {
        if count > value {
            value = *number;
        }
    }

    value
}

fn pig_latin(word: &String) -> String {
    let consonants = String::from("qwrtyplkjhgfdszxcvbnm");

    let mut first_consonant = String::from("");
    for char in word.chars() {
        if consonants.contains(char) {
            first_consonant = String::from(char);
            break;
        }
    }

    String::from(word) + &"-" + &first_consonant + &"ay"
}
