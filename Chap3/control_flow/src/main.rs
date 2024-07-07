fn main() {
    let number = 3;

    if number < 5 {
        println!("The condition is true\n");
    } else {
        println!("The condition is false\n");
    }

    multiple_ifs();
    if_let();
    // loop_ex();
    loop_ex2();
    loop_ex3();
    while_loop();
    array_iteration();
    array_iteration2();
    array_iteration3();
}

// example for else if
fn multiple_ifs(){
    let number = 6;
    if number % 4 == 0 {
        println!("The number is divisible by 4\n");

    } else if number % 3 == 0 {
        println!("The number is divisible by 3\n");
    }else if number % 2 == 0 {
        println!("The number is divisible by 2\n");
    } else {
        println!("The number is not divisible by 4, 3 or 2\n");
    }
}

// using if in a let statement
fn if_let(){
    let condition = true;
    let number  = if condition { 5 } else { 6 };

    println!("The number is {number}\n");
}

//using loop
// fn loop_ex(){
//     loop {
//         println!("again!");
//     }
// }

// returning values from loop
fn loop_ex2(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };

    println!("The result is {result}\n");
}

fn loop_ex3(){
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End Count = {count}\n");
}

// while loop example
fn while_loop(){
    let mut counter = 3;

    while counter != 0 {
        println!("count: {counter}");

        counter -= 1;
    }
    println!("LIFTOFF!!!\n");
}

//looping through an array using while
fn array_iteration() {

    let a = [10,20,30,40];
    let mut index = 0;

    while index < 4 {
        println!("the value is {}", a[index]);

        index +=1;
    }
    println!("\n");
}

// using for loop to iterate through an array
fn array_iteration2() {
    let a = [1,2,3,4];

    for element in a {
        println!("the value is {element}");
    }
    println!("\n");
}

// reverse array iteration using for loop
fn array_iteration3() {

    for number in (1..4).rev() {
        println!("number: {number}");
    }
    println!("LIFTOFF!!!\n");
}