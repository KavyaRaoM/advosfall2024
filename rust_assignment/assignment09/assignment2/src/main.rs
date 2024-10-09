fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integer numbers
    let numbers = [12, 5, 15, 8, 23, 30, 7, 9, 25, 16];

    // Iterate through the array and analyze each number
    for &num in numbers.iter() {
        // Check if the number is even or odd
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        // Check divisibility and print Fizz, Buzz, or FizzBuzz
        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz");
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz");
        }
    }

    // Use a while loop to find and print the sum of all numbers in the array
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Use a loop to find and print the largest number in the array
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number is: {}", largest);
}
