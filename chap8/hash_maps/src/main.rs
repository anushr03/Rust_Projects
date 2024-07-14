use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"),50);

    //accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    //accessing values using for loop
    for (key, value) in &scores {
        println!("key: {key}, Value: {value}");
    }

    // Adding a Key and Value Only If a Key Isn’t Present

    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"), 10);
    scores2.entry(String::from("Yellow")).or_insert(40);
    scores2.entry(String::from("Blue")).or_insert(40);

    println!("{scores2:?}");

    // Updating a Value Based on the Old Value

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;

    }
    println!("{map:?}");

}
