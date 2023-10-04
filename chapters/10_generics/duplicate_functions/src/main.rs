fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    
    let largest = find_largest_number(&number_list);
    println!("The largest number is {:?}", largest);
    
}

fn find_largest_number(list: &Vec<i32>) -> i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = &number;
        }
    }

    *largest
}
