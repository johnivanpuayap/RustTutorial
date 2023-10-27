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


    // primitive data types
    // scalar types vs compound types
    // scalar types are single values

    // by default an integer is created as i32
    //signed integer could also be i8, i16, i32, i64, i128 (number of bits that is used)
    // you can use negatives on a signed integer
    let a:i32 = -2;
    println!("{}", a);

    // unsigned integer
    let b:u32 = 2;
    println!("{}", b);

    // i8 vs u8
    // u8: 0 to 255
    // i8: -128 to 127
    
    //floating point values
    // f32 and f64
    // by default f64 is used
    let c: f64 = 10.9;
    println!("{}", c);

    //boolean
    let true_or_false = false;
    println!("{}", true_or_false);

    // char
    // use single quotes
    let letter: char = 'a';
    println!("{}", letter);

    // compound types are multiple values

    // tuples
    // tuples are immutable by default but you can make them mutable by using mut
    // you can't changed the type of a tuple
    let mut tup = (1, true, 's');
    println!("{} {} {}", tup.0, tup.1, tup.2);

    tup.0 = 2;

    println!("{} {} {}", tup.0, tup.1, tup.2);

    let tup2:(i8, bool, char) = (1, true, 's');
    println!("{} {} {}", tup2.0, tup2.1, tup2.2);

    // arrays
    // arrays are immutable by default but you can make them mutable by using mut
    // arrays must have the same type
    // you can't print the whole array
    let arr = [1, 2, 3, 4, 5];
    for &element in &arr {
        println!("{}", element);
    }

    // to type an array you must use [type; size]
    let mut arr2:[i8; 5] = [5, 4, 3, 2, 1];
    arr2[0] = 1;
    for &element in &arr2 {
        println!("{}", element);
    }




    // strings


}