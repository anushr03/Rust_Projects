// Initializing a struct

struct User {
    active: bool,       //field
    username: String,   //field
    email: String,      //field
    sign_in_count: u64, //field
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("anushr03"),
        email: String::from("anush.rathore382@gmail.com"),
        sign_in_count: 1,
    };

    println!("Current email id: {}", user1.email);
}