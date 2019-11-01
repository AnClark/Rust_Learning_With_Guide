fn main() {
    another_function(5, 6);

    // Use a scope to assign value.
    // This is a little bit similiar to instant function in Ecmascript.
    //
    // Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a statement,
    //    which will then not return a value.
    let _x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // Call a function returning just a number
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// NOTICE: In function signatures, you must declare the type of each parameter.
//          This is a deliberate decision in Rust’s design.
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// A simple funtion with expression returning
// By default, Rust's function returns the last expression value,
//  so you won't need to specify its return type.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}