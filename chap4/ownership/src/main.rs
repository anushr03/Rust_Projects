fn main() { // s is not valid here since not yet declared
    let s = "hello"; // s valid from this point forward
    // do something with s
    println!("{s}\n");
    appending_string();
}// the scope of s is over from this point forward

// appending to a string
fn appending_string() {
    let mut s = String::from("Hello");
    s.push_str(", world!\n");
    println!("{s}\n");
}