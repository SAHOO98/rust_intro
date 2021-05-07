// Used for iterations

pub fn run(){
    let mut count:i8 = 0;
    // Infinite loop
    loop{
        count+=1;
        println!("{}", count);

        if count == 10{
            break;
        }
    }

    count = 1;
    // While loop
    println!("\nfizzbuzz\n");
    while count<=100 {
        
        if count %15 ==0{
            println!("fizzbuzz");
        }else if count %5 ==0 {
            println!("buzz");
        }else if count % 3==0 { 
            println!("fizz");
        }else{
            println!("{}", count);
        }
        count+=1;
    }

    // For range
    println!("\nODD-EVEN\n");
    for number in 1..11{
        println!("{}", number&1 == 1 );
    }
}
