fn main() {
    let mut s = String::from("hello");     // s comes into scope
    let s1 = String::from("hello");     // s1 comes into scope

    let len = calculate_length(&s1);    // s1 keeps ownership because we only
                                        // pass a reference to s1

    change(&mut s);                     // pass a mutable reference to change()
    println!("{}", s);

    println!("The length of '{}' is {}.", s1, len); // s1 is still in scope here
}   // Here, s1 goes out of scope, along with len.

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.

fn change(some_string: &mut String) {   // Receives a mutable reference
    some_string.push_str(", world!");   // add to some_string
}