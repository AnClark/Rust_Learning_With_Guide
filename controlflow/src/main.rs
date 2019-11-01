fn main() {
    // 1. If expressions - basic
    // NOTICE: Other than C/C++, ECMAScript or Python, Rust's if expression
    //          only supports an ACTUAL bool value.
    //         This is because Rust will not automatically try to convert
    //          non-Boolean types to a Boolean.
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number {              // This is not allowed!
    if number != 0 {
        println!("number was something other than zero");
    }

    // 2. If expressions - else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 3. Using `if` in a `let` statement
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // 4. Loop - returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // 5. Loop - The usage of `break`
    //            Expression after `break` will be this loop's return value
    let result = loop {
        break 99;
    };

    println!("Now the result is {}", result);

    let result = loop {
        break {
            let x = 100;
            x * 2
        };
    };

    println!("Now the result is {}", result);

    // 6. While
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // 7. While - loop over an array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // 8. For - use `for` to iterate a collection, just like Python
    // NOTICE: This will be faster and safer than `while`.
    println!("** Use for to iterate");

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // 9. For - use Range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
}
