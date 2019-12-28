
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(userName:String, email:String) -> User
{
    let user = User {
        username: userName.clone(),
        email: email,
        sign_in_count: 1,
        active: true,
    };
    user
}

struct Point(i32, i32, i32);
struct Color(f64, f64, i32);


#[derive(Debug)]
struct Rectangle {
    height:u32,
    width:u32,
}

impl Rectangle
{
    fn area(&self, other:&Rectangle) -> u32 {
        self.height * self.width
    }
}

fn main() {

    let test_username = String::from("test_username");
    let test_email = String::from("test_email");
    let user = build_user(test_username, test_email);
    let test_point = Point(32, 32, 32);
    let test_color = Color(64.0, 64.0, 32);

    let rect:Rectangle = Rectangle {
        height:32,
        width:32,
    };

    
    let rect_2:Rectangle = Rectangle {
        height:32,
        width:32,
    };


    println!("here is test {}", rect.area(&rect_2));
   
}