//understanding difference between mutable and immutable variables
fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    shadowing();
    floating();
    numeric_operation();

}
// example to showcase shadowing
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is {x}");

}

//Floating point
fn floating(){

    let x = 2.0003;
    let y: f32 = 2.32323232;

    println! ("The value of x is {x} and value of y is {y}");

}

fn numeric_operation(){

    let add = 3+2;
    
    let subtraction = 5.44 - 10.21;

    let product = 4*10;

    let quotient = 56.32/4.33;
    let truncated = -5 / 3;
    let remainder = 43%5;

    println!("Addition = {add} \n");
    println!("subtraction = {subtraction} \n");
    println!("product = {product} \n");
    println!("quotient = {quotient} \n");
    println!("truncated = {truncated} \n");
    println!("remainder = {remainder} \n");

}
