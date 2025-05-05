fn main(){
 println!("Hello World!");
 total()
}

// Primitive data types
fn total(){
 // integer 
 let a : i16 = -32768;
 let b : u128 = 255;
 println!("Signed integer: {}" , a);
 println!("Unsigned integer: {}" , b);

 //float // f32 , f64
 let float: f32 = 3.12;
 println!("Float: {}",float);

 //bool // true,false
 let y: bool = true;
 println!("is it raining? {}",y);

 //char 
 let words: char = 'g';
 println!("{}", words);
}