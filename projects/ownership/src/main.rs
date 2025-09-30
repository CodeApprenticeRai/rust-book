fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world!");

    // println!("{s}");

    let s = String::from("hello");

    takes_ownership(s);

    // s.push_str(" world!");
    println!("{s}");

}

fn takes_ownership(some_string_ptr: String) {
    println!("String variable: {}", some_string_ptr);
}
