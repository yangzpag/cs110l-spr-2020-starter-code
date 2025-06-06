// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)

    let len = secret_word.len();
    let mut guessted_result: Vec<char> = std::iter::repeat('-').take(len).collect();
    let mut guessted_letters: Vec<char> = vec![];
    let mut try_times = NUM_INCORRECT_GUESSES;

    println!("Welcome to CS110L Hangman!");
    while try_times > 0 && guessted_result != secret_word_chars {
        println!(
            "The word so far is {}",
            guessted_result.iter().collect::<String>()
        );
        println!(
            "You have guessed the following letters:{}",
            guessted_letters.iter().collect::<String>()
        );
        println!("You have {} guesses left", try_times);

        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let guess_vec: Vec<char> = guess.chars().collect();
        // WARN: 直接获取，忽略报错
        let guess_ch = guess_vec.get(0).unwrap();
        guessted_letters.push(*guess_ch);
        let mut flag = false;

        for (i, ch) in secret_word_chars.iter().enumerate() {
            if ch == guess_ch {
                guessted_result[i] = *ch;
                flag = true;
                try_times = NUM_INCORRECT_GUESSES;
            }
        }
        if !flag {
            try_times -= 1;
            println!("Sorry, that letter is not in the word");
        }
        println!("");
    }

    if guessted_result == secret_word_chars {
        println!(
            "Congratulations you guessed the secret word: {}!",
            guessted_result.iter().collect::<String>()
        );
    } else {
        println!("Sorry, you ran out of guesses!");
    }
}
