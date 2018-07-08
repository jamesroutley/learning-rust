use std::env;
use std::process;


// Given a string S and a set of words D, find the longest word in D that is a subsequence of S.

// Word W is a subsequence of S if some number of characters, possibly zero, can be deleted from S
// to form W, without reordering the remaining characters.

// Note: D can appear in any format (list, hash table, prefix tree, etc.

// For example, given the input of S = "abppplee" and D = {"able", "ale", "apple", "bale",
// "kangaroo"} the correct output would be "apple"

// The words "able" and "ale" are both subsequences of S, but they are shorter than "apple".  The
// word "bale" is not a subsequence of S because even though S has all the right letters, they are
// not in the right order.  The word "kangaroo" is the longest word in D, but it isn't a
// subsequence of S.

fn main() {
    // let s = "abppplee";
    // let words = vec!["able", "ale", "apple", "bale"];
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage();
    }
    let sequence: &String = &args[1];
    let words: Vec<String> = args[2..].to_vec();
    for word in &words {
        println!("{}", is_subsequence(sequence, word))
    }
}

fn is_subsequence(sequence: &String, word: &String) -> bool {
    // Pull out each string's chars:
    let sequence_chars: Vec<char> = sequence.chars().collect();
    let word_chars: Vec<char> = word.chars().collect();

    // The sequence can't contain the word if the word is longer
    if word_chars.len() > sequence_chars.len() {
        return false
    }

    let mut sequence_ptr = 0;
    let mut word_ptr = 0;
    loop {
        // We've run out of sequence characters to check
        if sequence_ptr >= sequence_chars.len() {
            return false;
        }

        // We've got to the end of the word - it's a match
        if word_ptr >= word_chars.len() {
            return true;
        }

        // Characters match, let's check the next one in the word
        if sequence_chars[sequence_ptr] == word_chars[word_ptr] {
            sequence_ptr += 1;
            word_ptr += 1;
            continue
        }

        // Characters don't match - let's check the next char in the sequence
        sequence_ptr += 1;
    }
}

fn usage() {
    eprintln!("usage: longest-subsequence <sequence> <words>");
    process::exit(1)
}
