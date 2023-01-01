struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

      let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main_cor() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


struct AlwaysEqual;

fn main_empty() {
    let subject = AlwaysEqual;
}

