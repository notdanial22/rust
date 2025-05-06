//Shadowing
//Shadowing is the process of using the same variable name again while reassigning a different value to it.
// shadowing is not same as mutability.

// fn main() {
//     let x = 5;
//     let x = x * x;

//     {
//         let x = x * x;
//         println!("The value of x in the inner scope is: {}", x);
//     }

//     println!("The value of x in the outer scope is: {}", x);
// }

// another example

fn main() {
    let spaces = "    ";
    let spaces = spaces.len();
    // here we are assigning two different types first is a string and second is a integer
    // if we used mutability it would cause an error because we cannot assign two different types to the same variable
    println!("{} spaces", spaces);
}
