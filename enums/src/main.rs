// structs give a way of grouping data,
// enums give us a way of saying that value can be one of the possible values

enum IpAddrKind {
    V4(String),
    V6(String),
}

// enums can also have an implementations
impl IpAddrKind {
    fn print_it(&self) {
        println!("Just printing static data...")
    }
}

fn main() {
    let four = IpAddrKind::V4(String::from("0.0.0.0"));
    let six = IpAddrKind::V6(String::from("::1"));

    four.print_it();
    six.print_it();

    let value = plus_one(Some(2));
    println!("value is: {}", value.expect("none value returned"));

    let value = plus_one(None);
    println!("value is: {}", value.expect("none value returned"))
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("Got value: {i}");
            Some(i + 1)
        }
    }
}
