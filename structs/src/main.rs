// struct definitions are similar to golang

// normal data structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs can be created
struct Color(i32, i32, i32);

// we can declare structs without any data whatsover
// struct AlwaysEqual;

// we can store pointers as data type of structs but requires
// lifetypes features of the rust language
// struct User {
//     active: bool,
//     username: &str, // missing lifetype specifier
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // all functions inside impl are associated functions
    // we can have functions that don't need self reference. (like; static methods)
    // we have already used String::from , which is one of those
    // rust doesn't have new keyword built in

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // we can even declare methods with same properties, rust knows if its a method call or property calls
    // with the added parenthesis
    fn width(&self) -> bool {
        self.width > 0
    }

    // Self types are an alias, they have same data type as the impl struct
    // :: syntax is used for both associated functions as well as namespaces
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// there can be multple impl blocks, no problem
impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Dipesh Dulal"),
        email: String::from("nilkantha.dipesh@gmail.com"),
        sign_in_count: 10,
    };
    user1.email = String::from("new_email@new.com");
    println!("user: {}", user1.username);

    let user2 = build_user(user1.email.clone(), user1.username.clone());
    println!("email: {}, username: {}", user2.email, user2.username);

    let user3 = User {
        email: String::from("new_email@new.com"),
        ..user1
    };
    print_user(&user3);

    let black = Color(0, 0, 0);
    println!("Color rgb({}, {}, {})", black.0, black.1, black.2);

    // area -----
    let rectangle_x = 10;
    let rectangle_y = 5;
    println!("{}", area_normal(rectangle_x, rectangle_y));

    let rectangle: (u32, u32) = (10, 5);
    println!("Area: {}", area_tuple(rectangle));

    let rectangle = Rectangle {
        width: 10,
        height: 5,
    };
    println!(
        "Rectange: {:#?}, Area: {}",
        rectangle,
        area_rectangle(&rectangle)
    );

    println!("Rectangle area: {}", rectangle.area());

    if rectangle.width() {
        println!("width greater than zero has been provided")
    }

    let rectangle2 = Rectangle {
        width: 9,
        height: 2,
    };
    let rectangle3 = Rectangle {
        width: 90,
        height: 20,
    };
    println!("R1 can hold R2? {}", rectangle.can_hold(&rectangle2));
    println!("R1 can hold R3? {}", rectangle.can_hold(&rectangle3));
    // dbg!(rectangle); // debug macro to print out some values if necessary

    let new_square = Rectangle::square(15);
    println!("area of square is: {}", new_square.area())
}

fn print_user(user1: &User) {
    println!(
        "User: {}, {}, {}, {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area_normal(x: u32, y: u32) -> u32 {
    x * y
}

fn area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area_rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
