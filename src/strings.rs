//Primitive str = Immutable fixed-lenght string somewhere in memory
//String  = Mutable, stored in heap.

pub fn run (){
    let game = "Hitman: Absolution."; //Fixed length string
    let mut access = String::from("Dripping ");

    println!("{}", access);

    // Pushes a single character
    access.push('\u{1F32E}');
    println!("{}", access);

    //Pushes string
    access.push_str(" wet");
    println!("{}", access);

    //Capacity in bytes
    println!("Capacity: {} bytes", access.capacity());

    //is_Empty check on strings
    println!("Is \"access\" empty: {}", access.is_empty());

    // Contains
    println!("Does \"access\" contains wet: {}", access.contains("wet") );

    // Replace
    println!("Replaced :{}", access.replace("Dripping", "Drenching"));

    //Split in white space
    for tokens in access.split_whitespace(){
        println!("{}", tokens);
    }
    println!("{:?}", (game, access, game.len()));

    //Create string with capacity
    let mut stri = String::with_capacity(5);
    stri.push('a');
    stri.push('x');

    assert_eq!(2, stri.len());
    assert_eq!(5, stri.capacity());
    println!("{}", stri);
}
