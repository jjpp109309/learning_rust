mod front_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order()
    }

    fn cook_order() {}

    // creating a pub enum makes all of its variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


use crate::front_house::hosting;
pub fn eat_at_restaurant() {
    // call add to waitlist with ABSOLUTE path
    crate::front_house::hosting::add_to_waitlist();

    // call add to waitlist with RELATIVE  path
    front_house::hosting::add_to_waitlist();

    // call add to waitlist with USE clause
    hosting::add_to_waitlist();
    
    // order breakfast in summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // change mind about the bread we want

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // next line wont compile 'cause field is not public
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod customer {
    use crate::front_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// bring name as if it had been defined as pub
pub use crate::front_house::hosting;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
