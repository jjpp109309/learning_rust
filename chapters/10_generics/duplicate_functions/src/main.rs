fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let number_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    
    let largest = find_largest_number(&number_list);
    println!("The largest number is first list is {:?}", largest);

    let largest_2 = find_largest_number(&number_list_2);
    println!("The largest number in second is {:?}", largest_2);
    
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
