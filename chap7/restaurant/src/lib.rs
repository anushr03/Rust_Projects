mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) ->Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("peaches"),
            }
        }
    }


    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

//Paths for Referring to an Item in the Module Tree
/*Paths can take two forms, Absolute path and relative path
Absolute Path: An absolute path is the full path starting from a crate root; 
for code from an external crate, the absolute path begins with the crate name,
and for code from the current crate, it starts with the literal crate.

Relative Path: A relative path starts from the current module and uses 
self, super, or an identifier in the current module.
*/


pub fn eat_at_restaurant() {
    //Absolute Path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I would like to have {} toast please", meal.toast);
}

