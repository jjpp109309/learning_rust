use std::fs::File;
use std::io::{self, Read};

// main function can also resturn Err
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let userame_file_result = File::open("hello.txt");

    let mut username_file = match userame_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

}

fn read_username_form_file_2() -> Result<String, io::Error> {
    // another way using the ? opperator which implements the above match

    let mut username2 = String::new();
    
    File::open("hello.txt")?.read_to_string(&mut username2)?;

    Ok(username2)

}
