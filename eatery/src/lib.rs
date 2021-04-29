#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    fn seat_at_table() {}
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}


// Hoist serve_order into crate scope
use front_of_house::serving::serve_order;

mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup, 
        Salad
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order(){}
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    // Order breakfast with rye toast
    let mut meal = back_of_house::Breakfast::summer(("rye"));
    // Change the toast variety!
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // Invalid access to the seasonal_fruit var which is not public
    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
