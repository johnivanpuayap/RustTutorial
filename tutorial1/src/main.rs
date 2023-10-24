fn main() {
    
    println!("Hello, world!");
    
    // rust is a statically and strongly typed language
    // implicit type declaration
    // let mut x = 4;
    let x = 4;
    println!("x is : {}", x);
    // by default variables are immutable so we can't change them, use mut to make them mutable or use let to redeclare them
    // x = 5;
    let x = 5;
    println!("x is : {}", x);
    
    // name shadowing
    // rust has block scoping
    {
        // let x = 2;
        // println!("x is : {}", x);
        let x = x - 2;
        println!("x is : {}", x);
    }

    let x = x + 1;
    println!("x is : {}", x);

    // explicit type declaration
    let y: i32 = 5;
    println!("y is : {}", y);

    let y = "hello";
    println!("y is : {}", y);

    // constants
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("Seconds in Minutes is : {}", SECONDS_IN_MINUTE);
    // you can't reassign/redefine a constant
