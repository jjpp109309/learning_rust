fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // nothing happens till we use the iterator

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

