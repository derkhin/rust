// Rust has three kinds of loops: loop, while, and for.

fn main() {

    // basic_loop()
    // loop_return_value()
    // multiple_loop()
    // conditional_while_loop()
    // while_loop_over_array()
    // for_loop_over_array()
    // count_down()
    
}

// you can place the break keyword within the loop to tell the program when to stop executing the loop.
// and continue which tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration

fn basic_loop() {
    loop {
        println!("hello world inside loop!!");
    }
}
// Returning Values from Loops

fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result from loop {result}");
}

// Loop Labels to Disambiguate Between Multiple Loops

fn multiple_loop() {
    let mut counter = 0;
    'counting_up: loop {
        println!("counter {counter}");
        let mut remaining = 10;
        loop {
            println!("remaining {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("counter {counter}");
}

// Conditional Loops with while

fn conditional_while_loop() {
    let mut counter = 4;
    while counter != 0 {
        println!("counter : {counter}");
        counter -= 1;
    }
    println!("Finished")
}

//note: for example, if you changed the definition of the a array to 
// have four elements but forgot to update the condition to while index < 5, the code would panic

fn while_loop_over_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("element {index} : {}", a[index]);
        index += 1;
    }
    println!("Finished")
}

// Looping Through a Array Collection with for

fn for_loop_over_array() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("element : {element}")
    }
}

// countdown in for loop

fn count_down() {
    for number in (1..20).rev() {
        println!("number: {number}");
    }
}
