//understanding difference between mutable and immutable variables
fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}\n");

    shadowing();
    floating();
    numeric_operation();
    tuple_eg();

}
// example to showcase shadowing
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is {x}\n");

}

//Floating point
fn floating(){

    let x = 2.0003;
    let y: f32 = 2.32323232;

    println! ("The value of x is {x} and value of y is {y}\n");

}

// understanding numeric operations
fn numeric_operation(){

    let add = 3+2;
    
    let subtraction = 5.44 - 10.21;

    let product = 4*10;

    let quotient = 56.32/4.33;
    let truncated = -5 / 3;
    let remainder = 43%5;

    println!("Addition = {add} ");
    println!("subtraction = {subtraction} ");
    println!("product = {product} ");
    println!("quotient = {quotient} ");
    println!("truncated = {truncated} ");
    println!("remainder = {remainder} \n");

}

// tuple example: Tuple is of fixed length.
// group together a number of values with variety of types 
fn tuple_eg(){

    let tup: (i32,f64,u8) = (-500, 3.2, 6);
    let(x,y,z) = tup;
    println!("Value of x is {x}, y is {y}, z is {z}");

    let tup2: (i32,f64,u8) = (-500, 3.2, 6);
    let five_hundred = tup2.0;
    let three_point_two = tup2.1;
    let six = tup2.2;

    println!("Five hundred = {five_hundred}, three.two = {three_point_two}, six = {six}\n")

}
