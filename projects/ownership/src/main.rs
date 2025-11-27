fn main() {
    let mut s = String::from("Hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");

    println!("{some_string}")
}

// Think of it as a clay ball that is owned by A
// If A gives you to use it - You can't change the shape 
// but use the clay ball and return it to A
// But if A says that you can change this Clay ball to any shape
// Then you can change the shape of it
// But if you can change the shape then no other person will take it

// In technicality this prevents data races at compile time
// But if no mutation references - there can be many
// And no continuous immutable and mutable references
// And no dangling - only valid references