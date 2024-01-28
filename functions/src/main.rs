// Rust code uses snake case as the conventional style for function and variable names,
// Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
// let x = (let y = 6); Invalid in rust - statement does not return any value.

fn second_function() {
    println!("this is from second function");
}

// Parameters
fn parameter_function(x: i32) {
    println!("The value of parameter_function x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    let y = {
        let x = 3; // statement
        x + 1   // experssion
    };
    println!("The value of y is: {y}");
    println!("The measurement is: {value}{unit_label}");
}

// Functions with Return Values

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("this is from main function");
    second_function();
    parameter_function(5);
    print_labeled_measurement(5, 'h');
    let returned_from_five_function = five();
    let added_number = plus_one(6);
    println!("the return value from five() function is {returned_from_five_function}");
    println!("the added number from plus_one() function is {added_number}");
}