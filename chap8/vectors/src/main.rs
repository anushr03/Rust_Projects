fn main() {

    let v1: Vec<i32> = Vec::new(); // creating a new Vector
    let v2 = vec![1,2,3]; // another way to create

    // updating a vector
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);


    //Reading elements of the Vectors
    // Two ways to do so, via indexing or using the get method

    // Indexing method
    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {third}");

    let third2: Option<&i32> = v4.get(2);
    match third2 {
        Some(thrid) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //iterating over the values in an Vector
    let mut v5 = vec![100,39,40];
    for i in &v5 {
        println!("{i}");
    }

    // Here we are adding 50 to each value in the vector. Make sure to see the 
    // that the iteration is over a mutable vector and the * in front of i
    for i in &mut v5 {
        *i += 50;
    }


    // creating an enum that will store vectors
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell:: Int(3),
        SpreadsheetCell:: Float(3.2),
        SpreadsheetCell:: Text(String::from("blue")),
    ];


}








