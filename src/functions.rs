
fn greetings(greet:&str, name: &str){
    println!("{} {}, nice to meet you!", greet, name);
    
}

fn add (n:i32, m:i32)->i32{
    let result = m+n;
    return result;
}

pub fn run(){
    greetings("Ms", "Olivia");
    let sum = add(65, 4);
    println!("Sum = {}", sum);

    //Closure--kinda like a lambda in python
    let shift:i32 = 1;
    let xor = |x:i32, y:i32| (x^y) >> shift;

    println!("(5 Xor 8) >> {} = {}",shift,  xor(5,8));
}