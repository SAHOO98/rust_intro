//Tuples are a collection of different values from different datatypes
//Maximum size of tuple is 12 elements

pub fn run(){
    let person : (&str, &str, u8) = ("Saptarshi", "IEM", 69);

    println!("{} studies in {} and has an Id of{}", person.0, person.1, person.2);

}