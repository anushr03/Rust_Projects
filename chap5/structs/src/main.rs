// Initializing a struct

struct User {
    active: bool,       //field
    username: String,   //field
    email: String,      //field
    sign_in_count: u64, //field
}

fn main() {
    // Way 1 to using the struct
    let mut user1 = User {
        active: true,
        username: String::from("anushr03"),
        email: String::from("anush.rathore382@gmail.com"),
        sign_in_count: 1,
    };
    println!("\n");
    println!("Current email id: {}", user1.email);
    user1.email = String::from("anush.rathore@gmail.com");
    println!("New email id: {}", user1.email);

    let user2 = build_user("andyatwood382@gmail.com".to_string(),"andyatwood".to_string());
    println!("user2 email id: {}\n", user2.email);

    //Way 3 to using a struct
    let user3 = User {
        email: String::from("rocking@gmail.com"),
        ..user1

    };
    // In the example above, because we took the values of user1 into user3, 
    //the ownership of user1's username has gone to user3.
}

// Way 2 to using a struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

//tuple structs
struct Color()