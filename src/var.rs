// Variables hold rpimitive data or reference to data
// Variables are immutable(const) by default
// Rust is a block scoped language

pub fn run(){
    let  name  = "Saptarshi";
    let  mut age =  22;
    println!("My name is {} and I am {}", name, age);
    age = 23;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID:u8 = 69;
    println!("ID : {}", ID);

    // Assign multiple vars
    let (item0, version) = ("C", 99);
    println!("{} is in version {}", item0, version);




}