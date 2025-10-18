use std::io;
pub fn fibo_finder() {
    println!("Fibonacci game!");
    
    loop {
    println!("Which number in the fibonacci sequence would you like to see?");

      let mut user_no = String::new();

        io::stdin()
            .read_line(&mut user_no)
            .expect("Failed to read line");


    match user_no.trim().parse::<u32>() {
        Ok(number) => {

        let mut first_num = 1;
        let mut second_num = 0;
        let mut result = 0;


    for i in (0..number -1) {
         
        result = first_num + second_num;

        let old_first_num = first_num;

        second_num = old_first_num;
        
        first_num = result;
    }
            println!("{:?}", result);
        }
        Err(_) => println!("Error: That was not a valid whole number. Please try again.")
    };

    };

}
