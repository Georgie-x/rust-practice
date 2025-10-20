mod number_guess;
mod twelve_days;
mod fibo_finder;
mod trivia_api;
use std::io;

fn main() {
    println!("Welcome to my Rust practice exercises");
    let choices = vec![
        "ng = number guess",
        "td = twelve days",
        "ff = fibo finder",
        "ta = trivia api",
    ];
    let mut selected_module = String::new();

    loop {
        selected_module.clear();

        println!("\nPlease choose which code to run (e.g., ng): {}", choices.join(" | "));

        io::stdin()
            .read_line(&mut selected_module)
            .expect("Failed to read line. Please try again.");

        match selected_module.trim() {
            "ng" => number_guess::number_guess(),
            "td" => twelve_days::twelve_days(),
            "ff" => fibo_finder::fibo_finder(),
            "ta" => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(trivia_api::run_trivia_quiz_cli());
            }
            _ => {
                println!("Invalid choice. Please enter a valid abbreviation.");
                continue
            },
        };
    }
}