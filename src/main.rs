mod number_guess;
mod twelve_days;
use std::io;

fn main() {
    println!("Welcome to my Rust practice exercises");
    let choices = vec!["number guess", "twelve days"];
    let mut selected_module = String::new();
    
    loop{
    
        selected_module.clear();

    println!("Please choose which code to run: {:?}", choices.join(" "));
     
        io::stdin()
            .read_line(&mut selected_module)
            .expect("Please try again");

        match selected_module.trim() {
            "number guess" => number_guess::number_guess(),
            "twelve days" => twelve_days::twelve_days(),
            _ => continue,
        };
    }
}



