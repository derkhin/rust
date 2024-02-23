#[derive(Debug)] 
struct Rectangle {
    length: u32,
    breadth: u32,
}

impl Rectangle {
    //methods
    fn calculate_area_rectangle(&self) -> u32 {
        self.length * self.breadth
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.breadth > other.breadth
    }
}

// associated functions
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            breadth: size,
            length: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        breadth: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        breadth: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        breadth: 60,
        length: 45,
    };

   let sq = Rectangle::square(3);

    // println!("{:#?}", rect);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Area of rect1 {} px`", rect1.calculate_area_rectangle());
    println!("Area of rect1 {:#?} px`", sq);
}
