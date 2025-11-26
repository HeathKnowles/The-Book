fn main() {
    let x = 5;
    
    let x = x + 1;

    // Inner Scope Shadowing
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Shadowing helps us by not making us create separates variable names
    let spaces = "  ";
    let spaces = spaces.len();

    // But variable type is not mutable based on the above

    // All mathematical ops are possible in Rust
    // Rust can include Characters other than ASCII - As it represents unicode

    // Compound Types
    // 1 - Tuple - Different Types are possible
    let tup : (i32, char, f64) = (500, 'c', 7.9);
    let (x,y,z) = tup;
    println!("The value of {x}, {y}, {z}");
    // or
    println!("The value of {}, {}, {}", tup.0, tup.1, tup.2); // Not Supported as declarable prints

    // 2 - Array - Same Type Only
    let a = [1,2,3,4,5,6];
    let b : [i32;5] = [1,2,3,4,5];
    let c = [3;5]; // [3,3,3,3,3]

    let first = a[0];
    // No values after n - 1 (Overflowed Index)

}

