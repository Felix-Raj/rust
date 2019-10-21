struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
/*
struct WithRefs {
    username: &str,  // but requires lifetime to be specified
}
*/
fn build_user(email:String, username:String) -> User {
    User{
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

// tuple structs
// no names associated with fields
struct Point(i32, i32, i32);

// unit structs
// no fields
// useful - want to implement some trait, but don't have data that you want
//   to store in the type itself

fn main() {
    // create instance of struct
    let user1 = User{
        username: "Felix Raj".to_string(),
        email: String::from("some@one.com"),
        sign_in_count: 0,
        active: true,
    };

    // dot notation to read values
    println!("User name is {}", user1.username);

    let mut user2 = User{
        username: String::from("Another User"),
        email: "another@some.com".to_string(),
        sign_in_count: 24,
        active: false,
    };
    // dot notation to write, but should be mutable
    user2.email = String::from("new@email.com");

    let user3 = build_user(String::from("Some@One.com"),
                       "SomeOne".to_string());

    // create instance with struct update syntax
    let user4 = User{
        email: "New@Email.com".to_string(),
        username: "NewUser".to_string(),
        .. user3
    };

    let point1 = Point(1, 3, 4);
    println!("Point 1 x {}", point1.0);

    #[derive(Debug)]  // help to use specifier `:?` in print. Try w/o this
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle{  // implementation block
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        // *associated functions* - don't take self a parameter
        // associated with struct, they are functions not methods as they
        // don't have an instance of struct to work with
        fn square(size: u32) -> Rectangle {
            Rectangle { width:size, height:size }
        }
    }
    impl Rectangle {
        fn perimeter(&self) -> u32 {
            // this is also fine, can have multiple impl blocks
            self.width + self.height
        }
    }
    let rect1 = Rectangle {width:30, height:90};
    let rect2 = Rectangle {width: 10, height: 20};
    let sq3 = Rectangle::square(34);
    println!("rect1 is {:?}", rect1);  // Debug o/p format - :?
    println!("rect1 is {:#?}", rect1);  // Helps with complex structs
    println!("Area of rect1 {}", rect1.area());
    println!("rect1 can hold rect2 {}", rect1.can_hold(&rect2));
    println!("Square {:#?}", sq3);
}