use std::io;

fn main() {
    
    loop{

    println!("Enter the temperature in Celsius");

    let mut cel = String::new();

    io::stdin()
        .read_line(&mut cel)
        .expect("Failed");


    let cel: f32 = match cel.trim().parse() {
        Ok(num) => num,
         Err(_) => {
            // Просто завершаем программу, если ввод некорректен
            return;
        }
        };
    

    let f = cel * 1.8 + 32.0;

    println!("Temperature in Fahrenheit: {}", f);
    }
}
