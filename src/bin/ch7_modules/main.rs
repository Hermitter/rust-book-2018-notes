// Modules visual aid
// They are similar to file paths
// crate(src/main.rs, src/lib.rs)
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// Private & Public scope
pub mod backpack {
    // Private
    fn open() {}

    // Public
    pub fn grab() {}
}

fn main() {
    // Or relative paths
    crate::backpack::grab();

    // Modules can be called from absolute paths
    backpack::grab();

    println!("hello");
}

// Children can access parent module with super::
// Children have access to ancestors' privates (lol)
fn open1() {}
mod bag1 {
    fn open2() {}

    mod bag2 {
        fn unzip() {
            super::super::open1();
            super::open2();
        }
    }
}

// Struct fields are private by default
pub mod waffle_house {
    pub struct Menu {
        pub toast: String,    // accessible
        secret_toast: String, // unaccessible
    }

    // Struct initializer functions are required to make any struct with private fields
    pub fn make_menu(bread: &str) -> Menu {
        Menu {
            toast: String::from(bread),
            secret_toast: String::from("Sourdough"),
        }
    }
}

// Enums have their fields either all public or private
mod apple_bees {
    // All public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // All private
    enum Dessert {
        ice_cream,
        cake,
    }
}

// use: A way to bring paths into scope
mod car {
    pub mod engine {
        pub fn start() {}
        fn stop() {}
    }
}

use crate::car::engine; // absolute path, but can also be relative 'use car::engine;'
use car::engine::start; // any item can be brought into scope
use car::engine::start as start_car; // 'as' can rename a path

fn drive() {
    // both calls are the same
    engine::start();
    start();
    start_car();
    // engine::stop(); // still private
}

// Structs & Enums should, by convention, be included in an entire path
// unless there's a name conflict in the same scope
use std::collections::HashMap;
fn placeholder() {
    let y: HashMap<i32, i32> = HashMap::new();
    let x = HashMap::<i32, i32>::new(); // turbo fish ::<> will be covered in the future
}

// importing multiple paths from the same crate
use std::{array, borrow, cmp::Ordering};

// importing a crate path and path to item inside
use std::io::{self, Write}; // imports io & Write

// importing everything from a path into scope. (mainly used in tests module)
use std::collections::*;

// Files can be imported as modules
mod some_file;

fn get_treasure() {
    let x = some_file::HashMap::<i32, i32>::new();
    some_file::chest::treasure();
}

// Folder can have their code imported as well
mod some_folder;

fn get_treasure_again() {
    some_folder::chest::treasure();
}
