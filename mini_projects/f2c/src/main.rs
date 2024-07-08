use std::io;
fn main() {
    
    println!("Enter the temperature in Farenheit: ");
    let mut farenheit = String::new();

    io::stdin()
        .read_line(&mut farenheit)
        .expect("failed to read line");

        let farenheit: f32 = farenheit.trim().parse().expect("Please type a number!");

    println!("The temperature is: {farenheit}F");

    let  celcius = (farenheit - 32.0) * 5.0/9.0;
    println!("The temperature in Celcius is: {:.2}C", celcius);

}
