// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
// Assume the input is alpha numeric

use std::collections::HashMap;

fn main() {
    let text = String::from("let us first run to the apple forge");
    let mut latin_text = String::from("");

    let mut vowel = HashMap::new();
    vowel.insert("a", 0);
    vowel.insert("e", 0);
    vowel.insert("i", 0);
    vowel.insert("o", 0);
    vowel.insert("u", 0);

    for word in text.split_whitespace() {
        let (start, rest) = word.split_at(1);
        if vowel.contains_key(start) {
            latin_text.push_str(&format!("{}-hay ", word));
        } else {
            latin_text.push_str(&format!("{}-{}ay ", rest, start));
        }
    }

    println!("{}", latin_text);
}
