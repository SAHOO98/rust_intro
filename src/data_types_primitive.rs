/*
Primitive Types:-
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/ 

// Rust is a statistically typed language, however it can infer the type based upon value assignation and how we use it.
pub fn run(){
    //Default : i32
    let i = 25;

    //Default: f64
    let j = 65.5;

    //Explicit
    let y: i64 = 45484516626;

    println!("i = {}, j = {}, y = {}",i, j, y);

    //Maximum for i64
    println!("Max. of i64={}", std::i64::MAX);

    //Minimun for i64
    println!("Min. of i64={}", std::i64::MIN);

    //Maximum for u64
    println!("Max. of u64={}", std::u128::MAX);

    //Boolean
    let is_active:bool = false;

    //Get Boolean from expression
    let  is_ten_greater_than_four = 10>4; 

    //Characters
    let chracter: char = 's';
    let emoji = '\u{1F48B}';

    println!("{:?}", (i, j, y, is_active, is_ten_greater_than_four, chracter, emoji));
}