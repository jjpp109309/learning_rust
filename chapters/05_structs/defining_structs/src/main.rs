struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let username = String::from("abcde");
    let email = String::from("abcde@fghi.com");
    let user2 = build_user(username, email);

    // create a struct from another struct
    let user3 = User {
        email: String::from("another@example.com"),
        // active: user1.active,
        // username: user1.username,
        // sign_in_count: user1.sign_in_count,
        ..user1 // use the same values from user 1
    };
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // username: username,
        // email: email,
        // params have same names as fields, we use shorthand notation
        username,
        email,
        sign_in_count: 1,
    }
}
