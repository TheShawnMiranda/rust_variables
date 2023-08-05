fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x += 6;
    // println!("The value of x is: {x}");
    // const MAX_POINTS: u32 = 100_000;
    // println!("The value of MAX_POINTS is: {MAX_POINTS}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");


    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }


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