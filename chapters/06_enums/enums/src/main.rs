enum IpAddrKind {
    V4,
    V6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32)
}

// same encoding of messages but with structs would be messy :(
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// we can also define methods on enums
impl Message {
    fn call(&self) {
        // some awesome method!
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // storing address data using structs
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    // storing address data using enums (same concept)
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // enums variants can have different types, impossible with strucs
    let home_types = IpAddrTypes::V4(127, 0, 0, 0);
    let loopback_types = IpAddrTypes::V6(String::from("::1"));


    // enums with methods!
    let m = Message::Write(String::from("hello"));
    m.call();

    // The option enum: The rust way to represent something or nothing
    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;

}
