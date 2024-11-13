fn main() {
    let my_user = build_user(String::from("barberdt@gmail.com"), String::from("barberdt"));
    let my_other_user = User {
        username: String::from("davis"),
        email: String::from("davis@teachable.com"),
        sign_in_count: 4,
        ..my_user
    };

    print_user(&my_user);
    print_user(&my_other_user);

    let black = RGBColor(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("The R value of black is {}", black.0);
    println!("The z value of origin is {}", origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!(
        "User with username {} and email {} has signed in {} {}",
        user.username,
        user.email,
        user.sign_in_count,
        pluralize_string(&mut String::from("time"), user.sign_in_count),
    );
}

fn pluralize_string(s: &mut String, n: u64) -> &mut String {
    if n > 1 {
        s.push_str("s");
    }

    s
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct RGBColor(i32, i32, i32);
struct Point(i32, i32, i32);
