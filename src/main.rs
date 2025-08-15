fn main() {
    // let s = String::from("hello");

    // take_ownership(s);

    // let x = 5;

    // makes_copy(x);

    // //println!("{s}");
    // println!("{x}");

    // //references
    // let s1 = String::from("hello world");

    // let len = calculate_len(&s1);

    // println!("The length of {s1} is {len}.");

    //mutable references

    let mut s_mut = String::from("Hello");

    s_mut.push_str(", world!");

    println!("{s_mut}");

    change(&mut s_mut);
    println!("{}", s_mut);
}

// fn take_ownership(some_string: String) {
//     println!("{some_string}");

// }

// fn makes_copy(some_integer: i32){
//     println!("{some_integer}");
// }

// fn calculate_len(s2: &String) -> usize {
//     s2.len()
// }
fn change(some_string: &mut String) {
    some_string.push_str(", Welcome to the new world of Rust!!");
}