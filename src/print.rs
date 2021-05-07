pub fn run(){
    //print to console
    println!("Hello from print.rs file.");    

    //basic formatting
    println!("{} is a jerk from {}.", "John", 1969);

    //Positional arguments
    println!("{0} is good at socialising. {0} is bad at handling tough problems. He like to play {1}.","Hamel", "2048");

    //Named arguments
    println!("{name} like to eat {food}", name = "Septarmit", food="icecream.");
     
    //Placeholder traits
    println!("Binary={:b}, Hexadecimal={:x}, Octal={:o}", 0xff,255,255);

    //Placeholder trait
    println!("{:?}", (12, true, 25.69));

    //Basic Math
    println!("10+69={}",10+69);


}