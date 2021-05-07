// Basically if-else

pub fn run(){
    let age:u8 = 52;
    let check_id = true;
    let knows_person = false;

    if (age>21 && check_id) || knows_person {
        println!("Alcohol available!");
    } else if age<21 && check_id  {
        println!("Mountain Dew available!");
    }else{
        println!("Show your ID.");
    }

    //Short hand if, no ternary operator
    let a:u8 = 10;
    let b:u8 = 6;
    let c:u8 = 52;
    let grestest:u8 = if a>b{if a>c {a} else {c}} else{if b>c {b} else {c}};
    println!("Greatest of {}, {} and {} = {}", a, b, c, grestest);

}