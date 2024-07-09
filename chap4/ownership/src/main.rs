fn main() { // s is not valid here since not yet declared
    let s = "hello"; // s valid from this point forward
    // do something with s
    println!("{s}\n");
    appending_string();
    cloning();

    let mut word = String::from("Hello");
    change(&mut word);

    multiple_imut_references();
    let  word2 = String::from("New World!");
    let sliced_word = slice_eg(&word2);
    println!("{sliced_word}\n");


}// the scope of s is over from this point forward

// appending to a string
fn appending_string() {
    let mut s = String::from("Hello");
    s.push_str(", world!\n");
    println!("{s}\n");
}

fn cloning(){
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}\n");
}

// Mutable References
fn change(some_word: &mut String) {
    some_word.push_str(", World2!");
    println!("{some_word}\n");
}

// example with multiple immutable references
fn multiple_imut_references(){
    let  s = String::from("Hello3!");
    let s1 = &s;
    let s2= &s;

    println!("s = {s}, s1 = {s1}, s2 = {s2}\n");
}

// slicing
fn slice_eg(s: &String) -> &str {

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]


}