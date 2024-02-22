fn main() {
    let sentence=String::from("hello world order");
    let word = first_word(&sentence); 
    // sentence.clear(); // this empties the String, making it equal to ""
    println!("First Word: {word}");

    // String Slices
    let hello=&sentence[0..5]; // [inclusive,exclusive]
    let world=&sentence[6..11];
    println!("{hello} {world}");
}

fn first_word(s : &String)-> &str {
    let s_bytes=s.as_bytes();
    for (i, &item) in s_bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


