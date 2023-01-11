#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: false,
        sign_in_count: 0,
    };
    user1.email = String::from("user1@example.com");
    user1.username = String::from("user1");
    user1.active = true;
    user1.sign_in_count = 1;
    println!("user1 active is: {:?}", user1);

    let user2 = build_user(
        String::from("user2@example.com"),
        String::from("user2aaaaaaa")
    );
    println!("user2 is: {:?}", user2);

    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user2
    };
    println!("user3 is: {:?}", user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black is: {:?}", black);
    println!("origin is: {:?}", origin);

    let subject = AlwaysEqual;
    println!("subject is {:?}", subject);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
