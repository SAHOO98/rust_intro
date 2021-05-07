mod print;
mod var;
mod data_types_primitive;
mod strings;
mod tuples;

fn main() {
    print::run();
    println!("\n=================================\n");
    var::run();
    println!("\n=================================\n");
    data_types_primitive::run();
    println!("\n=================================\n");
    strings::run();
    println!("\n=================================\n");
    tuples::run();
    println!("\n=================================\n");

}   
