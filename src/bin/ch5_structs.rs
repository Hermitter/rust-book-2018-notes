fn main() {
    // struct example
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let person = User {
        username: String::from("Tim"),
        email: String::from("tim@hotmail.com"),
        sign_in_count: 23,
        active: true,
    };

    println!("{}", person.username);

    // values can be changed
    let mut person = User {
        username: String::from("Tim"),
        email: String::from("tim@hotmail.com"),
        sign_in_count: 23,
        active: true,
    };

    person.email = String::from("tim@gmail.com");
    println!("{}", person.email);

    // functions can return structs
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }
    let new_person = build_user("t@gmail.com".to_string(), "tim".to_string());

    // functions can use shorthand syntax to create struct. Argument must be the same name
    fn build_user2(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    let new_person = build_user2("t@gmail.com".to_string(), "tim".to_string());

    // Update syntax can copy a struct's fields into a new struct
    let mut person_copy = User {
        email: String::from("Tim@yahoo.com"),
        username: String::from("Timmy"),
        ..new_person
    };

    // Tuple structs. Struct that look like tuples
    // Both variables are considered different types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 1, 2);
    let origin = Point(0, 1, 2);
    println!("{} {}", black.1, origin.1);

    // Struct don't need to have fields (unit-like struct covered in ch.10)
    struct Animal {};

    // Struct can store reference but they need a specified lifetime (covered in ch.10))
    // Code below will not compile.
    // struct User {
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    //     active: bool,
    // }

    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };

    // Example program for struct usage
    #[derive(Debug)] // let's us print in debug mode
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    println!("{:?}", rect1); // ? means to print in debug mode
    println!("{:#?}", rect1); // you can add more options to change the output

    // methods
    // they are functions tied to a struct type
    #[derive(Debug)]
    struct Rectanglee {
        width: u32,
        height: u32,
    }

    impl Rectanglee {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn multi_args(&self, rect: &Rectanglee) -> bool {
            println!("{}", rect.area());
            true
        }

        fn new_rect(width: u32, height: u32) -> Rectanglee {
            Rectanglee { width, height }
        }
    }

    let rect1 = Rectanglee {
        width: 30,
        height: 50,
    };
    let rect2 = Rectanglee { ..rect1 };
    let rect3 = Rectanglee::new_rect(12, 15);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let conditional = rect1.multi_args(&rect2);
    println!("{}", conditional);

    // multiple impl blocks can exist
    impl Rectanglee {
        fn nothing_much(&self) {}
    }
}
