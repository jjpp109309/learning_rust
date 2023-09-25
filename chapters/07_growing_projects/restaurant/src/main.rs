// idiomatic to bring enums & structs into scope with full path
use std::collections::HashMap;

// exception: when bringing two items with same name
use std::fmt;
use std::io;

// exception avoided using aliases
use std::io as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn function1() -> fmt::Result {
    // snip
}

fn function2() -> io::Result<()> {
    // snip
}

fn function3() -> IoResult<()> {
    // snip
}
