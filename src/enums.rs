
enum Movement{
    up, down, left, right
}

fn movement(m :Movement){
    match m{
        Movement::up => println!("Moving Up!"),
        Movement::down => println!("Moving Down!"),
        Movement::left =>  println!("Moving Left!"),
        Movement::right =>   println!("Moving Right!"),
    }

}

pub fn run(){
    let man1 = Movement::up;
    let man2 = Movement::down;
    let man3 = Movement::left;
    let man4 = Movement::right;

    // movement(man1);
    movement(man3);
    // movement(man2);
    // movement(man4);
}