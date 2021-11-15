// struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体
struct AlwaysEqual;

fn build_user(email: &str, username: &str) -> User {
    return User {
        email: email.to_string(),
        username: username.to_string(),
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    // 普通结构体
    let mut user1 = build_user("someusername123", "someuseremail@example.com");
    user1.email = String::from("anotheremail@example.com");
    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };
    println!("{}", user1.email);
    // 元组结构体
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 类单元结构体
    let subject = AlwaysEqual;
}