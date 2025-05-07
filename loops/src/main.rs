// loops in rust
// fn main() {
//     let mut count = 0;
//     let result = loop {
//         count += 1;
//         println!("again!");
//         if count == 5 {
//             break count * 2;
//         }
//     };
//     println!("The result is {}", result);
// }

// while loops
// fn main() {
//     let mut count = 3;
//     while count > 0 {
//         println!("{}!", count);
//         count -= 1;
//     }
//     println!("while loop ended!")
// }

// for loops
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     for element in a {
//         println!("the value is: {}", element);
//     }
//     let alphabets = ['a', 'b', 'c', 'd', 'e'];
//     for alphabet in alphabets {
//         println!("the value is: {}", alphabet);
//     }
// }

// nested loops
fn main() {
    'outer: for x in 1..=3 {
        'inner: for y in 1..=3 {
            println!("x = {}, y = {}", x, y);

            if x == 2 && y == 2 {
                break 'outer;
            }
            if x == 4 {
                break 'inner;
            }
        }
    }
}
