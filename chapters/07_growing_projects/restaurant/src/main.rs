// idiomatic to bring enums & structs into scope with full path
use std::collections::HashMap;

// exception: when bringing two items with same name
use std::fmt;
use std::io;

// exception avoided using aliases
use std::io as IoResult;


// using use trees to import different levels of a hierarchy
// instead of
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

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
