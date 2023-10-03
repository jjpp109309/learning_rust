use std::fs::File;

fn main() {
    // try openning a file
    let greeting_file_result = File::open("hello.txt");
    
    // using Result enum to handle succeed and fail cases
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("problem openning the file {:?}", error),
    // };

    // handling different error kinds
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(er) => panic!("Problem creating the file {:?}", er),
            },
            other_error => {
                panic!("Problem openint the file {:?}", other_error);
            }
        }
    };

    // Another option using unwrap which return the Ok value or call panic
    // let greeting_file2 = File::open("hello2.txt").unwrap();

    // finally the expect method lets us write a panic message
    let greeting_file3 = File::open("hello2.txt")
        .expect("hello.txt should exist in this project");
        
}
