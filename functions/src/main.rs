fn main() {
    let _x: i32 = {
        let a = 10;
        let b = 40;
        a * b
    };
    println!(" value of x is {}", _x);
    let y: i32 = add(4, 6);
    println!(" value of y is {}", y);
    let height = 1.82;
    let weight = 70.0;
    let bmi = calculate_bmi(weight, height);
    println!(" BMI is {:.2}", bmi);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
