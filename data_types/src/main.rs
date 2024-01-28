fn main() {
    // Primary scaler types integers, floating-point numbers, Booleans, and characters.

    // Scaler Types

    // Integers default type is i32
    // signed - Inclusive -(2^(n - 1)) to 2^(n - 1) - 1 where n is the number of bits.
    // unsigned  0 to 2^(n - 1) .

    let _num_1: i8 = 20;
    let _num_2: u8 = 20;

    let _num_5: i16 = 20;
    let _num_6: u16 = 20;

    let _num_3: i32 = 200;
    let _num_4: u32 = 100;

    let _num_7: i64 = 20;
    let _num_8: u64 = 20;

    let _num_9: i128 = 20;
    let _num_10: u128 = 20;

    let _num_9: isize = 20;
    let _num_10: usize = 20;

    // specifying type in value itself

    let _num_11 = 20u8;
    let _num_12 = 20_u8;
    let _num_13 = 20.5_f64;

    // Floating-Point - Rust has two primitive types for floating-point numbers with signed, default type is f64

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Boolean Type - Booleans are one byte in size.
    
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Character Type
    // we specify char literals with single quotes, as opposed to string literals, which use double quotes
    // Rust’s char type is four bytes in size

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '🚀';

    // Integer Literals
    // Decimal	-> 98_222, 1000000000 -> 1_000_000_000 // same
    // Hex -> 0xff
    // Octal ->	0o77
    // Binary -> 0b1111_0000
    // Byte (u8 only) -> b'A'

    // Numeric Operations

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    let _remainder = 43 % 5;

    // Compound Types - Group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // Tuple 

    //  Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let _tup: (i32, u32, f64) = (20, 30, 12.5);

    // destructure a tuple value

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // Access a tuple element directly by using a period "."

    let _five_hundred = tup.0;

    // The tuple without any values has a special name, unit.
    // Expressions implicitly return the unit value if they don’t return any other value.

    let _tup = ();

    // Array 
    // Unlike a tuple, every element of an array must have the same type
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let _a = [1, 2, 3, 4, 5];

    // arrays are more useful when you know the number of elements will not need to change.
    // For example, if you were using the names of the month in a program,
    // you would probably use an array rather than a vector because you know it will always contain 12 elements

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // type defination in array
    // i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // same way of writing but in a more concise way

    let _a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    // Accessing Array Elements through index

    let _first = a[0];

    // Rust protects from invalid memory access through its check mechanism at runtime its a Rust’s memory safety principles in action
    // If the index is greater than or equal to the length, Rust will panic and gives index out of bound error
}
