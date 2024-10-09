use std::io;

// Function to check if the guess is correct, too high, or too low
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Hard-code the secret number
    let secret_number = 42;

    // Variable to track the number of guesses
    let mut guess_count = 0;

    // Start the guessing loop
    loop {
        // Create a mutable variable to store the user's guess
        let mut guess = String::new();

        // Prompt the user for input
        println!("Enter your guess:");

        // Read the input from the user and store it in the guess variable
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Convert the input to an integer
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue; // Skip to the next iteration if input is invalid
            }
        };

        // Increment the guess counter
        guess_count += 1;

        // Call the check_guess function
        let result = check_guess(guess, secret_number);

        // Use an if-else expression to print whether the guess was correct, too high, or too low
        if result == 0 {
            println!("Your guess of {} is correct!", guess);
            break; // Exit the loop if the guess is correct
        } else if result == 1 {
            println!("Your guess of {} is too high!", guess);
        } else {
            println!("Your guess of {} is too low!", guess);
        }
    }

    // Print how many guesses it took
    println!("It took you {} guesses.", guess_count);
}
