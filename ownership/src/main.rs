fn main() {
    let x = 10;
    let y = x; // copy - bcz rust have copy trait which directly store on stack i.e int,bool,char

    println!("value of x: {x}");
    println!("value of y: {y}");

    let s1 = String::from("hello");
    // let s2 = s1; // move - compile error

    let s2 = s1.clone();
    println!("value of s1: {s1}");
    println!("value of s2: {s2}");

    let sample_string = String::from("hello world");
    ownership_test(sample_string);
    // println!("from main fn : {sample_string}"); //error

    let mut s3 = String::from("Hello");
    // let s4: usize = give_and_take_ownership(s3);
    // println!("S3: {s3}"); s3 will moved instead we need to take refrence
    
    give_and_take_ownership(&mut s3);
    println!("S3: {s3}");

    let mut name = String::from("derkhin");
    let name_immutable_ref: &String = &name;
    let name2_immutable_ref: &String = &name;
    println!("{name_immutable_ref} {name2_immutable_ref}");
    let name_mutable_ref: &mut String= &mut name;
    name_mutable_ref.push_str("ctrl");
    println!("{name_mutable_ref}");

    let reference_to_nothing = dangle();

}


//error instead of refrence pass directly s
fn dangle() -> &String {  // String
    let s = String::from("hello"); 

    &s  // s
}

fn ownership_test(updated_value: String) -> String {
    println!("test: {updated_value}");
    updated_value
}

fn give_and_take_ownership(val: &mut String) {
    val.push_str(", World");
}
