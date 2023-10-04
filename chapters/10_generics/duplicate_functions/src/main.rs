use std::cmp::PartialOrd;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    
    let result = find_largest_number(&number_list);
    println!("The largest number is first list is {:?}", result);

    let largest_2 = find_largest_number(&number_list_2);
    println!("The largest number in second is {:?}", largest_2);

    // deduplication using generic data types
    let number_list_3 = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list_3);
    println!("The largest number in third list is {:?}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest number in char list is {:?}", result);
    // using the function with generic types
    let number_list = vec![34, 50, 25, 100, 65];
    
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    
    let result = largest(&char_list);
    println!("The largest char is {}", result);

}

fn find_largest_number(list: &Vec<i32>) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// without generics we have two functions
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// we have two functions with same code... lets use a generic type parameter
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
