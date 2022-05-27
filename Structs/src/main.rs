struct Color(i32, i32, i32);
struct Point(i32,i32,i32);

struct AlwaysEqual;

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let subject = AlwaysEqual;
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    let mut user1 = User{
        email: String::from("zeljkomtc@gmail.com"),
        username: String::from("Zeljo"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("zeljo@gmail.com"); // how to edit parts of a struct

    let user2 = User{
        /*active: user1.active,
        username: user1.username,
        sign_in_count: user1.sign_in_count, without struct update syntax*/
        email: String::from("person2@hotmail.com"),
        ..user1
    };
}


fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}