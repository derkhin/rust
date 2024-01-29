// If the condition isn’t a bool, we’ll get an error
// The error indicates that Rust expected a bool but got an integer. Unlike languages such as Ruby and JavaScript,
// Rust will not automatically try to convert non-Boolean types to a Boolean.
// You must be explicit and always provide if with a Boolean as its condition

fn main() {
    is_not_zero(0);
    multiple_condition(23);
    state_function()
}

fn is_not_zero(x: i8) {
    if x != 0 {
        println!("number was something other than zero");
    } else {
        println!("number is equal to zero");
    }
}

// Handling Multiple Conditions with else if
// Using too many else if expressions can clutter your code,
// so if you have more than one, you might want to refactor your code.

fn multiple_condition(number: u32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Using if in a let Statement
// The number variable will be bound to a value based on the outcome of the if expression.
// results from each arm of the if expression must be the same type
// If the types are mismatched, as in the following example, we’ll get an error:

fn state_function() {
    let num = if false { 5 } else { 0 };
    // let num = if false { 5 } else { "Six" }; // type mismatched we'll get an error
    println!("The value of num is {num}");
}


