use std::io;
pub fn fibo_finder() {
    println!("Fibonacci game!");
    println!("Which number in the fibonacci sequence would you like to see?");


      let mut user_no = String::new();

        io::stdin()
            .read_line(&mut user_no)
            .expect("Failed to read line");


    match user_no.trim().parse::<u32>() {
        Ok(number) => {

        let mut evens = 0;
        let mut odds = 1;
       

    for i in 1..number {
         let mut result = evens + odds;
        if i == number {
            println!("{:?}", result);
        } else if i % 2 == 0 {
            evens = i
        } else {
            odds = i
        }
    }
            println!("{:?}", result);
        }
        Err(_) => println!("Error: That was not a valid whole number. Please try again.")
    }




}
