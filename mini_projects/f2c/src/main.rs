use std::io;
fn main() {
    
    loop{

        println!("Enter the temperature in Farenheit: ");
        let mut farenheit = String::new();

        io::stdin()
            .read_line(&mut farenheit)
            .expect("failed to read line");

            let farenheit :f32 = match farenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };
        

        println!("The temperature is: {farenheit}F");

        let  celcius = (farenheit - 32.0) * 5.0/9.0;
        println!("The temperature in Celcius is: {:.2}C\n", celcius);
    }

}
