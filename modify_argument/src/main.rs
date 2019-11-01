// Learh how to modify parameters of a function
//
// Two situations:
//      1. Modify argument's value;
//      2. Modify the variable which a argument refers to
//
fn main() {
    //
    // Situation 1: I'm about to modify the variable "s" by calling the function `change_int`
    //
    let mut s = 1;
    println!("Original s = {}", s);

    // Call the function. Note that we must add `mut` and `&` (reference mark) when invoking.
    println!("** Modify the variable `s` by function!");
    change_fn_reference_value(&mut s);
    println!("Updated s = {}", s);

    //
    // Situation 2: I'm just about to modify the value of the argument, with variable which is used to 
    //               assign its value UNTOUCHED.
    //
    change_fn_argument_value_only(s);
    println!("Now s = {}. Isn't it unchanged?", s);
}

// [ Situation 1 ] The function modifying any arguments refered here
//
// NOTICE: You must add `&mut` before the type.
fn change_fn_reference_value(src: &mut u32) {
    *src = 9999;
}

// [ Situation 2 ] The function will just modify the argument "src"'s value. 
// Now, "src" is treated as a local variable.
//
// NOTICE: In this condition, you should add `mut` ahead of argument's name.
fn change_fn_argument_value_only(mut src: u32) {
    println!("** Now just change the argument value only");
    src += 1;
    println!("`src` add one! Now its value is: {}", src);
}

