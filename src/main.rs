fn main() {
    // The main loop
    loop {
        // Clears the screen and places the cursor at the top left
        print!("\x1B[2J\x1B[1;1H");

        // Title
        println!("++++++++++++++++++++++++++++++++++++++++");
        println!("Is it FIZZ or is it BUZZ? Or is it BOTH!");
        println!("++++++++++++++++++++++++++++++++++++++++\n");
        
        // Variables needed
        let mut input = String::new();
        let mut try_again = String::new();

        // Reading the user's input
        println!("Enter your number: ");
        get_user_input(&mut input);

        // Converting the input to u32
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // Checking if the value is fizz, buzz, or fizzbuzz
        if input % 3 == 0 && input % 15 == 0 {
            println!("FIZZ-BUZZ");
        } else if input % 3 == 0 {
            println!("FIZZ");
        } else if input % 15 == 0 {
            println!("BUZZ");
        } else {
            println!("NONE");
        }

        // Asking if the user wants to try again
        println!("\nDo you want to try again(Y/N)? ");
        get_user_input(&mut try_again);

        // Capatalizes the input
        try_again = try_again.to_uppercase();

        if try_again.trim() == "N" {
            break;
        }
    }

    println!("\n\nThanks for playing!");
}

fn get_user_input(user_input: &mut String) {
    std::io::stdin()
        .read_line(user_input)
        .expect("Failed to read input!");
}