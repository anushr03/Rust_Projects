fn main() {
    let mut s = String::new();

    let data = "Initial Contents";
    
    let s = data.to_string();

    // Updating a string
    let mut s2 = String::from("foo");
    let s3 = "bar";
    s2.push_str(s3);
    println!("s3 is {s3} ");

    //
    let s4 = String::from("Hello");
    let s5 = String::from("World");
    let s6 = s4 + &s5; // we reference s5 because thats how the signature of the method is made

    // using the format! macro

    let s7 = String::from("Tic");
    let s8 = String::from("Tac");
    let s9 = String::from("Toe");

    let s10 = format!("{s7}-{s8}-{s9}");
    println!("{s10}");
}
