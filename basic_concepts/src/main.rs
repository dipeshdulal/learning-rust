use std::io::stdin;

fn main() {
    // by default variables are mutable
    let x = 5;
    println!("value of x is {x}");

    // this is not possible, immutable variables cant be mutated
    // x = 6;
    // println!("value of x is {x}");

    // mut keyword used for mutability
    let mut x = 10;
    println!("x: {x}");

    x = 30;
    println!("x: {x}");

    // constants are always immutable and mutability cannot be changed for constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THS: {THREE_HOURS_IN_SECONDS}");
    // THREE_HOURS_IN_SECONDS = 30 impossible

    // shadowing:
    // can create a new variables with a same name as previous variable
    let x = 5;
    let x = x + 1; // look at how same names can be redeclared and reassigned by the power of shadowing

    {
        let x = x + 2;
        println!("value of inner is: {x}")
    }

    println!("value of outer is: {x}");

    let spaces = "     ";
    let spaces = spaces.len(); // look at how the data type can also be different, as the variable is new altogether
    println!("Got {spaces} spaces");

    // data types
    // compiler needs to know about the types before hand
    // let guess = "42".parse().expect("Not a number!");

    let emoji = '🤔';
    // rust has emoji support by default :)
    println!("Emoji: {emoji}");

    // compound types
    // tuples (1,2,3), destructuring to takeout values from a tuple
    // can access tuple elements by using `.` followed by index of the values we want to access
    let me = ("Dipesh", "Wesionary", 2023);
    println!("Tuple values are: {} {} {}", me.0, me.1, me.2);

    let a = [1, 2, 3, 4, 5];
    println!("enter index:");
    let mut index = String::new();

    stdin().read_line(&mut index).expect("failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("entered index should be a number");

    let element = a[index];
    println!("element at the index {index} is {element}");

    another_function()
}

// creating a function, functions should be snake cased
fn another_function() {
    println!("olaa from another function");

    let y = {
        let x = 3;
        x + 1 // no semicolons in this line, return value from this statement
              // semicolons means that they are expressions
    };

    // if and else can be used as statements
    let number = if y > 2 { y } else { 0 };
    println!("y is {y}, number is: {number}");
}
