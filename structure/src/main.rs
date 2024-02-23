fn main() {
    // struct_basic();   
    // struct_update();
    rectangles();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // 첫 매개 변수가 &self 이기 때문에 메소드
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // 첫 매개 변수가 &self가 아니기 때문에 연관 함수
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn rectangles() {
    //let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    let scale = 2;
    let rect1 = Rectangle {
        // width: dbg!(30 * scale),
        width: 30,
        height: 50
    };
    println!(
            "The area of the rectangle is {} square pixels.",
            // area(width1, height1)
            // area(rect1)
            // area(&rect1)
            rect1.area()
        );
    println!("rect : {:#?}", rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Can rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? {}", rect1.can_hold(&rect3));

    // assoiated function
    let square = Rectangle::square(12);
    println!("square : {} * {}", square.width, square.height);
}

// fn area(width: u32, height: u32) -> u32 {
//  width * height
// fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn struct_basic() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername1"),
        email: String::from("someusername1@example.com"),
        sign_in_count: 1
    };
    println!("email : {}", user1.email);
    
    user1.email = String::from("anotheremail@example.com");
    println!("email : {}", user1.email);

    let mut user2 = build_user(String::from("someusername2@example.com"), String::from("someusername2"));
    println!("email : {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn struct_update() {
    let user = build_user(String::from("someusername1@example.com"), String::from("someusername1"));
    println!("email : {}", user.email);
    let updated_user = update_user(String::from("another@example.com"), String::from("another"), user);
    // println!("email : {}", user.email); // error!!
    println!("email : {}", updated_user.email);
}

fn update_user(email: String, username: String, user: User) -> User {
    User {
        email,
        username,
        ..user
    }
}
