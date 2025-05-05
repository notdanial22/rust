// Ownership :
// 1.Each value in rust has an owner.

// fn main() {
//     let string = String::from("Ownership");
//     let len = calculate_length(&string); // here we are passing the reference of string to calculate_length function.
//     println!("The length of '{}' is {}.", string, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// 2. Each value has only one owner at a time.

// fn main() {
//     let s = String::from("hello");
//     let s1 = s; // here  ownership of s is transferred to s1
//                 // println!("{}", s); // will cause error because s is no longer valid.
//     println!("{}", s1);
// }

// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    let s = String::from("hello");
    println!("{} , {}", s, calculate_length(&s));
}

// fn lost_value(s: &String) {
//     println!("{}", s)
// } // here s is lost because it went out of scope.

fn calculate_length(s: &String) -> usize {
    s.len()
}
