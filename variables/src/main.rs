const MAX_POINTS: u32 = 100_000;

fn main() {
    // 1. Print out constants
    println!("The max points is: {}", MAX_POINTS);

    // 2. Mutable/Immutable variables
    let mut x = 5;
    println!("The value of x is: {}", x);

    // Now try to reassign
    x = 6;
    println!("The value of x is: {}", x);

    // 3. Shadowing - reuse the name
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    // 4. Shadowing - transfer
    let spaces = "        ";
    let spaces = spaces.len();

    println!("The length of spaces is: {}", spaces);

    // 5. Parse a value from a string
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("Number from string 'guess' is: {}", guess);

    // 6. The character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!(
        "The three characters you've input: {} {} {}",
        c, z, heart_eyed_cat
    );

    // 7. Tuple - create & access
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("Tuple tup item 0: {}", tup.0);
    println!("Tuple tup item 1: {}", tup.1);
    println!("Tuple tup item 2: {}", tup.2);
    // 8. Tuple - get the individual values out of a tuple
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // 9. Array
    // NOTICE: Rust's array is similiar to C/C++, with fixed length,
    //          unlike Python's list.
    //         In Python, you can use [] to access both tuple and list.
    //          but in Rust, [] is not allowed for tuple.
    let a = [1, 2, 3, 4, 5];
    println!("a[0]: {}", a[0]);

    // Explicitly tell Rust your type of array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // 10. Array - Initialize an array with a same value.
    //             just like `int a[5] = {0}` in C.
    let a = [3; 5];
    println!("a[4]: {}", a[4]);
    
}
