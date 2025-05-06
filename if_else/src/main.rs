// if else conditions in rust
fn main() {
    // let number = 18;
    // if number >= 18 {
    //     println!("You can drive the car.");
    // } else {
    //     println!("You can't drive the car.");
    // }

    //  multiple if else conditions in rust

    // let number = 23;
    // if number % 2 == 0 {
    //     println!("The number is divisible by 2.");
    // } else if number % 3 == 0 {
    //     println!("The number is divisible by 3.");
    // } else if number % 4 == 0 {
    //     println!("The number is divisible by 4.");
    // } else {
    //     println!("The number is not divisible by 2, 3, or 4.");
    // }

    //  if else in a let statement
    let is_allowed = true;
    let age = if is_allowed { 18 } else { 16 }; // remember we can not enter different types in a let statement.
    println!("You can drive the car if you are {} years old.", age);
}
