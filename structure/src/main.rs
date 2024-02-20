fn main() {
    // struct_basic();   
    struct_update();
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
