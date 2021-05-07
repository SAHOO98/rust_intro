// Vectors are resisable arrays like in C++
use std::mem;

pub fn run(){
    let mut numbers: Vec<u8> = vec![1,58,5,8,36];
    println!("{:?}", numbers);

    //Genral array manipulation
    println!("numbers[1] = {}", numbers[1]);
    numbers[3] = 85;
    numbers[2] = 69;
    //Add to vector
    numbers.push(85);
    numbers.push(125);
    println!("{:?}", numbers);
    println!("Lenght of vector : {}", numbers.len());
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // Slices
    let slice: &[u8] = &numbers[0..3];
    println!("Sliced array : {:?}", slice);

    //Loop through immutably
    for x in numbers.iter(){
        print!("{}", x);
    }
    print!("\n");

    //Loop through mutably
    for x in numbers.iter_mut(){
        *x /= 3;
    }
    print!("New Vector after mutation: {:?}", numbers);


}