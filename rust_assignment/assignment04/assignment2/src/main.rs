fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect(); // Split text into words
    let mut max_word = String::new();
    let mut max_count = 0;

    // Outer loop to check each word
    for word in &words {
        let mut count = 0;

        // Inner loop to count occurrences of the current word
        for other_word in &words {
            if word == other_word {
                count += 1;
            }
        }

        // Update max_word and max_count if the current word has more occurrences
        if count > max_count {
            max_word = word.to_string();
            max_count = count;
        }
    }

    (max_word, max_count) // Return the most frequent word and its count
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
