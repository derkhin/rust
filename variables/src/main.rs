fn main() {
    let x = 5; // Immutable by default
    let mut y = 5; //By mut keyword it become mutable
    y = 6;
    println!("the value of x: {x}");
    println!("the value of y: {y}");

    const Z: i32 = 10; // immutable - can't use mut with const and followed with the type i.e - i32
    println!("the value of Z: {Z}");

    //Shadowing
    let outter_scope_count = 10;
    let outter_scope_count = outter_scope_count + 10;
    {
        let outter_scope_count = outter_scope_count * 2;
        println!("the value of outter_scope_count inner scope: {outter_scope_count}");
    }
    println!("the value of outter_scope_count : {outter_scope_count}");

    //Shadowing is different from marking a variable as mut
    let spaces = 100;
    let spaces = 200;
    println!("the shadow spaces value : {spaces}");  // 200

   
    let mut spaces = 400; //compile error: consider making this binding mutable: `mut spaces`
    spaces = 300;
    println!("the shadowed spaces values : {spaces}");
}
