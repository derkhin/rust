// 192.168.1.1 -> type ip address
// ipV4, ipV6 -> variants of ipaddres

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// A Message enum whose variants each store different amounts and types of values

enum Message {  
    Quit, // has no data associated with it at all 
    Move {x:i32,y:i32}, //  has named fields, like a struct does.
    Write(String), // includes a single String.
    ChangeColor(i32, i32, i32) // includes three i32 values. like RGB
}

// below structs are realated to Message but not as a single type like above enum eg.

//--// 

// struct Quit;  // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

//--// 

impl Message {
    fn call(&self){
        println!("test")
    }
}

/*

Rust does not have nulls, but it does have an enum that 
can encode the concept of a value being present or absent. This enum is Option<T> 

*/

#[derive(Debug)]
enum City {
    Mumbai, 
    Hyderabad,  
    Kolkata,
    Noida
}

#[derive(Debug)]
enum Coin {
    Rupee(City),
    Two(City),
    Five(City),
    Ten(City),
    Twenty(City) 
}


fn main() {
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let host_ip = IpAddr::V6(String::from("192.168.1.1"));
    println!("V4: {:#?} V6: {:#?}", localhost, host_ip);

    let m = Message::Write(String::from("Hello World"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('c');
    let absent_number:Option<i32>  = None;


    let num1:i8=5;
    let num2:Option<i8> = None;
    let sum = num1 + num2.unwrap_or(10);  // 10 is default if num2 is None
    
    println!("{}",sum);
    println!("{}",get_coin_details(Coin::Twenty(City::Hyderabad)));
}

fn get_coin_details(coins: Coin) -> String {
    let (denomination, city) = match coins {
        Coin::Rupee(city) => (1, city),
        Coin::Two(city) => (2, city),
        Coin::Five(city) => (5, city),
        Coin::Ten(city) => (10, city),
        Coin::Twenty(city) => (20, city),
    };
    let details = format!("{} Rupee Coin is Made In {:#?}", denomination, city);
    details
}
