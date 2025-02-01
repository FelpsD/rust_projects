use std::io;

fn main() {
    loop {
        println!("Insert the convert method:");
        println!("1) Fahrenheit to Celsius");
        println!("2) Celsius to Fahrenheit");
        println!("3) Exit");
    
        let mut method = String::new();
    
        io::stdin()
        .read_line(&mut method)
        .expect("Failed to read line");
        
        let method: u32 = match method.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if method == 3 {break}

        println!("Insert the inicial value:");
    
        let mut value = String::new();
    
        io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");
    
        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
    if method == 1 {
        println!("{value} Fahrenheit in Celsius is {}", (value-32.0)*5.0/9.0);
    } else if method == 2 {
        println!("{value} Celsius in Fahrenheit is {}", value*9.0/5.0+32.0);
    } else {
        println!("Method don't exist!")
    }
    println!("");
    println!("");
    }
    

}

