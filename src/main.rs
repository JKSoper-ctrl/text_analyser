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

    for word in text.split_whitespace() {
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
            
            
            
            

    dbg!(word_map);

    dbg!(most_frequent_words);
    dbg!(most_frequent_word_count);


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