fn main() {
    // define a vector
    let v: Vec<i32> = Vec::new();

    // define a vector with macro that infers types
    let v2 = vec![1, 2, 3]; // type annotation not necesarry

    // add elements to a vector
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // reading elements of vectors
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);
}
