use std::io;

fn main() {
    println!("This is the Fahrenheit to Celsius Converter for the body temperature");

    
    println!("Enter the temperature in Fahrenheit")
    let mut ftemp = String::new();

    io::stdin()
        .read_line(&mut ftemp)
        .expect("Failed to read line");

    let ftemp: f32 = match ftemp
        .trim()
        .parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

    let ctemp: f32= (ftemp - 32.0) * 5.0/9.0;

    println!("The temperature in Celsius is: {ctemp}");
}