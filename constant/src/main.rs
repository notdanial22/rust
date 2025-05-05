fn main() {
    let mut _x = 2;
    // const mut y = 3;  // const cannot be mutable also const should be initialized with capital letter and we also need to specify the type.

    const MAX: i32 = 100;

    println!("{}", MAX);
    println!("{}", VALUE_OF_PI);
    println!("There are total {} days in a day", SECONDS_IN_A_DAY);
}

const VALUE_OF_PI: f64 = 3.14159; // global constant

const SECONDS_IN_A_DAY: u32 = 60 * 60 * 24;
