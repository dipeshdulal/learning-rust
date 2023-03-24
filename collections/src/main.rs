fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    for value in v {
        println!("V: {}", value);
    }

    // there is shorthand notation, macro for creating vectors
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    for value in v {
        println!("V: {}", value);
    }

    let mut v = vec![1, 2, 3];

    // there are two ways of accessing elements
    // let x = &v[10]; // no way to check if index is out of bounds, so this method is risky
    // println!("{x}");

    // we can use another method
    let third = v.get(2);
    match third {
        Some(value) => println!("Value: {}", value),
        None => println!("No value found"),
    }

    // can iterate over mutable reference of the elements
    for x in &mut v {
        *x += 100;
    }

    for x in &v {
        println!("Value: {}", x);
    }
}
