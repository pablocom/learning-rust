const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("three hours are {THREE_HOURS_IN_SECONDS} seconds");

    let mut x = 5;
    println!("x just initialized is {x}");
    
    x = x + 1;
    println!("x reassigned to {x}");

    let x = x + 27;
    println!("x shadowed with value {x}");    

    {
        let x = x * 69;
        println!("x shadowed within a scope {x}");   
    }

    println!("x value at scope exit {x}");   
}