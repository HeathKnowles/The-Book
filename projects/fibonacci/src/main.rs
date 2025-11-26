use std::io;

fn fibonacci(x: u32) -> u32{
    if x <= 0 {
        return 0;
    } else if x == 1 {
        return 1;
    } else {
        return fibonacci(x - 1) + fibonacci(x - 2);
    }
} 

fn main() {
    println!("Nth Fibonacci Number");

    println!("Enter the number: ");
    
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n
        .trim()
        .parse()
        .expect("Not an Integer");
    for n in 0..n+1 {
        print!("{} ", fibonacci(n));
    }
    println!("");
}
