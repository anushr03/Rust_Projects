mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order(){}
        fn serve_order() {}
        fn take_payment() {}
    }
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
}
