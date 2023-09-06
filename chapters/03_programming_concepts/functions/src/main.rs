fn main() {
    println!("Hello world!");

    another_fn();
    another_function(3301);
    another_function_2(5, 'h');
    statements_expressions();
    functions_w_return_values();
}

fn another_fn() {
    println!("Another function.");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn another_function_2(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements_expressions() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn functions_w_return_values() {
    let x = five();

    println!("The value of x is: {x}");
}
