// Arrays are of fixed length  and of one data type as in C/C++ or java.

use std::mem;

pub fn run(){
    let mut numbers: [u8; 5] = [5,8,5,4,2];
    println!("{:?}", numbers);

    //Genral array manipulation
    println!("numbers[1] = {}", numbers[1]);
    numbers[3] = 85;
    numbers[2] = 69;
    println!("{:?}", numbers);
    println!("Lenght of array : {}", numbers.len());
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Slices
    let slice: &[u8] = &numbers[0..3];
    println!("Sliced array : {:?}", slice);

}