fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x += 6;
    // println!("The value of x is: {x}");
    // const MAX_POINTS: u32 = 100_000;
    // println!("The value of MAX_POINTS is: {MAX_POINTS}");

    let x: u8 = 255;
    println!("The value of x is: {x}");


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