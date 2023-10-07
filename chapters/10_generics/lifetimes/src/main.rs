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
}
