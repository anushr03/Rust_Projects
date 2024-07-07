fn main() {
    println!("Hello, world!");

    another_function(5);
    multiple_arguments(6,'C');
    statments_expressions();
    let x = return_five();
    println!("The value of x is {x}");
}

// When using arguments, always declare the type
fn another_function(x:i32){

    println!("The value of x is {x} \n");
}

// Example to showcase multiple arguments
fn multiple_arguments(value: i32, unit: char){
    println!("The value is {value}, and the character is {unit}\n");
}

/* statements and expressions
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.
*/

fn statments_expressions(){

    let y = {
        let x = 6;
        x + 1
    };

    println!("The value of y is {y}\n");

}

//functions with return values
fn return_five() -> i32 {
    5
}
