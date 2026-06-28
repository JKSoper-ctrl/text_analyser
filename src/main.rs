/*
Brief - Word Frequency Analyser: Read a passage of 
text from the user, split it into words, then use a
`HashMap` to count occurrences. Report the most
frequent word (mode), the median word length, and a
sorted frequency table.
*/

use std::{collections::HashMap, io};

fn main() {
    let text = get_input("Paste text:");

    let mut word_map: HashMap<&str, u32> = HashMap::new();

    let mut most_frequent_words: Vec<&str> = Vec::new();
    let mut most_frequent_word_count: u32 = 0;
    let mut word_lengths: Vec<usize> = Vec::new();

    for word in text.split_whitespace() {
        word_lengths.push(word.len());
        
        let count = word_map.entry(word).or_insert(0);
        *count += 1;

        if *count == most_frequent_word_count {
            most_frequent_words.push(word);
        } else if *count > most_frequent_word_count {
            most_frequent_words.clear();
            most_frequent_words.push(word);
            most_frequent_word_count = *count;
        }
    }

    let median_word_length = get_median(&mut word_lengths);

    dbg!(word_map);
    dbg!(most_frequent_words);
    dbg!(most_frequent_word_count);
    dbg!(median_word_length);


    println!("{text}");

    

}

fn get_input(instruction: &str) -> String {
    let mut input_text = String::new();

    println!("{instruction}");

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input");

    input_text
}

fn get_median(numbers: &mut [usize]) -> f64 {
    numbers.sort_unstable();
    
    let numbers_length = numbers.len();
    let numbers_mid = numbers_length / 2;
    
    if numbers_length.is_multiple_of(2) { //if even number of elements
        ((numbers[numbers_mid - 1] + numbers[numbers_mid]) as f64) / 2.0 
    } else {
        numbers[numbers_mid] as f64
    }
}

/*
    word_lengths.sort_unstable();
    let word_lengths_length = word_lengths;
    let word_lengths_mid = word_lengths / 2;

    if word_lengths_length
            
*/