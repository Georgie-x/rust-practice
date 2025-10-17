mod number_guess;
mod twelve_days;
mod fibo_finder;
use std::io;

fn main() {
    println!("Welcome to my Rust practice exercises");
    let choices = vec!["1 = number guess", "2 = twelve days", "3 = fibo finder"];
    let mut selected_module = String::new();
    
    loop{
    
        selected_module.clear();

    println!("Please choose which code to run: {:?}", choices.join(" "));
     
        io::stdin()
            .read_line(&mut selected_module)
            .expect("Please try again");

        match selected_module.trim() {
            "1" => number_guess::number_guess(),
            "2" => twelve_days::twelve_days(),
            "3" => fibo_finder::fibo_finder(),
            _ => continue,
        };
    }
}



