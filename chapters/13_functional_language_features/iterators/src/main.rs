fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // nothing happens till we use the iterator

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // iterators that create other iterators
    let v1: Vec<i32> = vec![1, 2, 3];

    // v1.iter().map(|x| x + 1); // does not do anything
    let v2: Vec<i32> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

