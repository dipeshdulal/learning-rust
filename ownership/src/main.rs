fn main() {
    // let x = 5;
    // let y = x;
    // as integers are simple values, they are just pushed to the stack
    // no any case of ownership error will be shown here

    let s = String::from("hello");
    // let new_s = s;

    // now, since s1 is not a simple type, the equals statement donesn't make copy
    // of the variable and assign it to new_s, instead it just moves ownership of the varaible.

    // now we have ownership rules that says that
    // if variables ownership is lost, the variable is no longer accessible. (it's freed already)
    // so s cannot be used again.
    // println!("Value of s is: {s}"); // compiler error: value borrowed is already moved.

    // also, to aviod this error, we could deep copy the objects but there will be performance penalty.
    // So, in rust, it will nevery automatically create any deep copies.
    // anything automatic that rust does can be thought of as inexpensive operations.

    // we could clone by;
    let s2 = s.clone();
    println!("Now both {s} and {s2} can be used. 🎉");

    // as with variables, the ownership is transferred if the values are passed into any functions.
    take_ownership(s2);

    // println!("S2 is: {s2}"); // this is no longer vaid as ownership of s2 is now taken by take_ownership function.

    // now for simpler types that implements Copy trait.
    // this rule of ownership doesn't apply as the values are cloned.
    let x: u32 = 2;
    make_copy(x);
    println!("After copy {x}");

    let x = give_ownership(); // x variable now takes ownership of my_string variable
    println!("Ownership taken: {x}");

    let y = take_and_give_ownership(x); // take the ownership from x and return back ownership to y
    println!("Value of y: {y}");

    let (s2, len) = calculate_length(y);
    println!("Now string: {s2}, length: {len}"); // y is not accessible, ownership has transferred inside a function

    let len = calculate_length_w_pointer(&s2); // send pointer of s2 to function and not transfer ownership
                                               // because the ownership is not passed, the value of s2 wont be dropped and can be used again
    println!("Length is: {len}");
    let mut newS = s2; // using the passed string again, transfering ownership to mutable string newS
    change(&mut newS);
    println!("New value is: {newS}");

    // the mutable references have big restriction, borrowing references can only be done once.
    // change(&mut newS); // this works as the mutable reference has been returned back from borrowing in change function

    // let s = &mut newS;
    // let s_new = &mut newS;
    // println!("s {s} and s_new {s_new}")

    // this will cause an error, as we cannot borrow and use it more than once at a time.

    // one more thing, there canot be mutable and immutable reference of the same value
    let x = &newS;
    let y = &newS;
    // let z = &mut newS; // this doesn't work as we can't have both mutable and immutable references at the same time.
    // println!("{}, {}, {}", x, y, z)
}

// this function takes ownership of the string passed here.
fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

// make copy just makes a copy of the variable
fn make_copy(some_integer: u32) {
    println!("{}", some_integer);
}

// give ownership
fn give_ownership() -> String {
    let my_string = String::from("Hello owner"); // string comes into scope
    my_string // pass the ownership of the string to somebody else that calls this function
}

// this function takes ownership of a variable
// and then gives back the ownership of the variable
fn take_and_give_ownership(x: String) -> String {
    x
}

// calculate length of a string, returns tuple
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_w_pointer(s: &String) -> usize {
    // s.push_str("New things");  // modifying something that's borrowed donesn't work
    // just as variables are immutable by default the references are also immutable by default
    s.len()
}

// takes the mutable references of the variable and changes the variable
fn change(s: &mut String) {
    s.push_str("Added new things!!");
}
