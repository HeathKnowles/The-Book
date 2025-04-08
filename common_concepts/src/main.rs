fn main() {
    // Variables
    let x = 5;
    println!("The Value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT:u32 = 10_00_000;
    println!("The Subscribe Count = {}",SUBSCRIBER_COUNT);

    // Data Types

    // Scalar
        // Integers
            let a = 98_222;
            let b = 0xff;
            let c = 0o77;
            let d = 0b1111_0000;
            let e = b'A';

        // Floating point Numbers
            let  f = 2.0;
            let g = 3.0;

        //Booleans
            let t = true;
            let f = false;

        //Character
            let c = 'c';
            let x = 'x';
    
    // Compound
        // Tuple
            let tup = ("Let's get Rusty!",100_000);
            let (channel,sub_count) = tup;
            let sub_count = tup.1;

        // Array
            let error_codes = [100,122,321];
            let not_found = error_codes[1];
    
    // Control Flow
        if(a > b){
            println!("A is Greater than B");           
        } else {
            println!("A is lesser than B");
        }
    
    // Looping
        // Loop,While,for

    //Comments
        // Line comment

        /* 
            This is a Block Comment
        */

}

    // Functions
    fn my_function() {
        println!("Another Function");
    }

