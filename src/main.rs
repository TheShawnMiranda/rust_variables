fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x += 6;
    // println!("The value of x is: {x}");
    // const MAX_POINTS: u32 = 100_000;
    // println!("The value of MAX_POINTS is: {MAX_POINTS}");

    let x: u8 = 255;
    println!("The value of x is: {x}");

    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    // println!("The value of c is: {c}");
    // println!("The value of z is: {z}");
    // println!("The value of heart_eyed_cat is: {heart_eyed_cat}");


    //This is a tuple
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("The value of x is: {x}");
    println!("The value of z is: {z}");

    //Saying println!({tup.0}) will not work.
    let five_hundred = tup.0;

    println!("The value of tup.0 is: {five_hundred}");

    //Elements can be accessed by array[index].
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
              
    for month in months.iter() {
        println!("The value of month is: {month}");
    }

    //Here's an array that contains the number 3 five times for some reason.
    let array = [3; 5];
    println!("The value of array is: {:?}", array);

}

/*
    Variables are immutable by default. Using the mut keyword makes them mutable. You can change the value but not the type.
*/

/*
    Constants cannot be the result of a function but may be the result of a math caluclation.
    Constants are always immutable.
    Constants can be declared in any scope, including the global scope.
*/

/* 
    Shadowing is different from marking a variable as mut, because we’ll get a compile-time error
    if we accidentally try to reassign to this variable without using the let keyword or by trying to change it's type.
*/

/* 
    An integer can be signed (i.e. positive or negative) or unsigned (i.e. positive only).
    An i8 can store numbers from -128 to 127 inclusive, whereas a u8 can store numbers from 0 to 255 inclusive.
*/

/*
    In signed numbers, the first bit is used to indicate whether the number is positive or negative. 1 is negative, 0 is positive.
*/

/*
    Java's double is Rust's f64. Rust also has f32 which is Java's float.
*/

/*
    Boolean type is bool.
*/

/*
    Compound types can group multiple values into one type.
    Rust has two primitive compound types: tuples and arrays (fixed length).
    Tuples === (,) === Different types. Empty tuples are called unit.
    Arrays === [,] === Same types.
*/