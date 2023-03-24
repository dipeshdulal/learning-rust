use std::collections::HashMap;

fn main() {
    // strings are utf8 encoded.
    let mut hello = String::from("नमस्कार");

    // we can append to string
    hello.push_str("Another string");

    println!("hello: {}", hello);

    let another_str = String::from("Anotherrrr");
    let new_str = another_str + &hello; // internally add method is called something like add(self, &str)
                                        // ownership of anotherStr is not present now so another string can't be used.

    println!("New Str: {}", new_str);

    // format macro can also be used;

    let n = String::from("x");
    let new_str = format!("{new_str}-{n}"); // format doesn't take ownership of the variables
    println!("{}", new_str);

    // Rust strings doesn't support indexing

    // there are methods for iterating over strings
    let new_str = String::from("hello world");
    for x in new_str.chars() {
        println!("{x}");
    }

    for x in new_str.bytes() {
        print!("{x} ")
    }
    println!();

    let mut map = HashMap::new();
    let key = String::from("Test");
    map.insert(key, 1);
    println!("{:?}", map);
}
