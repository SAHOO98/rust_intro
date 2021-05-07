
// Traditional struct
struct Color{
    red : u8,
    green : u8,
    blue: u8
}

//Tuple struct
struct Point(i32,i32);

//Function implementation within structs
struct Complex{
    a : f64,
    b : f64, 
}

impl Complex{

    // Obtaining absolute value
    fn modulus(&self)->f64{
        let  z:f64 = self.a*self.a + self.b*self.b;
        return z.powf(0.5);
    }

    // Obtaining argument
    fn argument(&self)->f64{
        return self.a.atan2(self.b);
    }

    // Obtaing the complex number
    fn get_number(&self)->String{
        format!("{}+{}j", self.a, self.b)
    }

    //Change the number
    fn set_number(&mut self, a:f64, b:f64){
        self.a = a;
        self.b = b;
    } 

}
pub fn run(){
    let mut red  = Color{
        red :255,
        green:0,
        blue:0
    } ;

    println!("red : {}, green : {}, blue: {}", red.red, red.green, red.blue);   

    let mut p = Point(4,3);
    p.0 = 15;
    println!("x : {}, y : {}", p.0, p.1);   

    let mut z = Complex{
        a : 4.0,
        b : 3.0
    };
    println!("{}", z.get_number());
    println!("Modulus = {}, Argument = {}", z.modulus(), z.argument());

    z.set_number(12.0,-5.0);

    println!("{}", z.get_number());
    println!("Modulus = {}, Argument = {}", z.modulus(), z.argument());

}