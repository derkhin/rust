#[derive(Debug)]
struct User {
    name: String,
    email: String,
    count: u64,
    is_activate: bool,
}
#[derive(Debug)]
struct Color(i32, i32, i32); // color code (red, green and blue)
struct Pointer(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;


fn main() {
    let mut user1 = User {
        name: String::from("Irfan"),
        email: String::from("derkhin@gmail.com"),
        count: 1,
        is_activate: true,
    };
    user1.email = String::from("zerkhi@gmail.com");

    let user2 = build_user(String::from("user2"), String::from("user2@gmail.com"));

    let user3 = User {
        name: String::from("user3"),
        email: String::from("user3@example.com"),
        ..user2
    };

    println!("user1 {:#?} user2 {:#?} user3 {:#?}", user1, user2, user3);

    //Tuple Struct

    let rgb = Color(32, 43, 56);
    let origin = Pointer(0, 0, 0);
    println!("{:#?}", rgb);
    
    let subject = AlwaysEqual;
    println!("unit like struct : {:?}",subject);
}

fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        count: 1,
        is_activate: true,
    }
}
