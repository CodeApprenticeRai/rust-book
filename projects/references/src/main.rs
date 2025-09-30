fn main() {    
    // Reference Example
    // let s1 = String::from("Hello");
    // let len = calculate_length(&s1);
    // println!("The length of {s1} is {len}");


    // Borrow with Modify (does not compile)
    // change(&s1);


    // Borrow with Modify (compiles)
    // let mut s1 = String::from("Hello");
    // change(&mut s1);

    // Rust doesn't allow multiple mutable references to the same
    //  data at the same time (does not compile)
    // let mut s1 = String::from("Hello");
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{r1}, {r2}");


    // Borrowing in One Scope at a Time (compiles)
    // let mut s1 = String::from("Hello");
    // {
    //     let _r1 = &mut s1;
    // }
    // let _r2 = &mut s1;


    // Cannot borrow as mutable and immutable at the same time (does not compile)
    // let mut s1 = String::from("Hello");
    // let r1 = &s1;
    // let r2 = &s1;
    // let r3 = &mut s1;
    // println!("{r1}, {r2}, {r3}");

    // Can borrow as immutable multiple times (compiles)
    // let s1 = String::from("Hello");
    // let r1 = &s1;
    // let r2 = &s1;
    // println!("{r1}, {r2}");

    // Change what is referenced (does not compile)
    // let s1 = String::from("Hello");
    // let s2 = String::from("World");

    // let r1: &mut String = &mut s1;

    // println!("{r1}");

    // let r1 = &mut s2;

    // println!("{r1}");

    // Creating a dangling reference (does not compile)
    // let reference_to_nothing = dangle();

    let s = no_dangle();
    println!("{s}");
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(s: &String) {
//     s.push_str(" World");
// }

// fn change(s: &mut String) {
//     s.push_str(" World");
// }

// Cannot return reference to local variable (does not compile)
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// Can return  String (owned) type (compiles)
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}