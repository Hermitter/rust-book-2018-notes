fn main() {
    // std strings are mutable
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    // Simple values will automatically copy its value (or anything that implements copy)
    let x = 5;
    let y = x;

    // Complex values will be given a new owner
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}", s2); // s2 has the new value & s1 is dropped

    // Complex values can be copied with the clone method
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{} {}", s1, s2); // s1 is still valid

    // function inputs will also count and (move)ing owners
    fn eat_value(text: String) {}
    let s = String::from("hello");
    eat_value(s);

    // How to borrow a value without taking ownership
    let s1 = String::from("Hello!");
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    let x = calculate_length(&s1);
    println!("{}", s1);

    // How to modify a value without taking ownership
    let mut s1 = String::from("Hello!");
    fn change_string(s: &mut String) {
        s.push_str(" World");
    }
    change_string(&mut s1);
    println!("{}", s1);

    // There can only be one reference in scope if it is mutable (& mut)
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // r1 and r2 are no longer used after this point. They are now out of scope

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }

    // Slices are a reference to part of a collection
    // These slices have the same range
    let s = String::from("hello");
    let start_slice = &s[0..2];
    let start_slice = &s[..2];

    let end_slice2 = &s[2..s.len()];
    let end_slice2 = &s[2..];

    let entire_slice = &s[..];

    println!("{} {} {}", start_slice, end_slice2, entire_slice); //he llo hello

    // Return a slice
    let s = String::from("hello world");

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        println!("{:?}", bytes);

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }
    println!("{}", first_word(&s));

    // String can be used as a slice and &str is already slice
    fn takeLiteral(text: &str) {}

    let s = String::from("Hello World");
    takeLiteral(&s); // String
    takeLiteral("Hello World"); // &str

    // slices are not specific to string
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}
