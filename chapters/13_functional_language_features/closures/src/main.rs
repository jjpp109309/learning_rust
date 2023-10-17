use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };
    
    let user_pref1: Option<ShirtColor> = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!("The user with preference {:?}, gets {:?}", user_pref1, giveaway1);

    let user_pref2: Option<ShirtColor> = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!("The user with preference {:?}, gets {:?}", user_pref2, giveaway2);

    
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // }

    // functions with similar behavior to closures
    fn add_one_v1 (x: u32) -> u32 { x + 1 }
    let add_one_v1 = |x: u32| -> u32 { x + 1 };
    // let add_one_v2 = |x| { x + 1 };
    // let add_one_v3 = |x|  x + 1 ;

    // the compile infers types from usage
    let example_closure = |x| x;
    
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    // capturing references or owning
    let list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining the closure: {:?}", list);

    let mut borrows_mutability = || list.push(7);

    borrows_mutability();

    println!("After calling closure: {:?}", list);

    // forcing closure to take ownership of values
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // Rectangle
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

