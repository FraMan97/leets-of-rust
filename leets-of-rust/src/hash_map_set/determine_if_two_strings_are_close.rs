// Two strings are considered close if you can attain one from the other using the following operations:

//    Operation 1: Swap any two existing characters.
//        For example, abcde -> aecdb
//    Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
//        For example, aacabb -> bbcbaa (all a's turn into b's, and all b's turn into a's)

// You can use the operations on either string as many times as necessary.

// Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.
use std::collections::HashMap;

pub fn determine_if_two_strings_are_close(word1: &String, word2: &String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let mut m1 = HashMap::new();
    let mut m2 = HashMap::new();

    for c in word1.chars() { *m1.entry(c).or_insert(0) += 1; }
    for c in word2.chars() { *m2.entry(c).or_insert(0) += 1; }

    // Condizione 1: Devono avere gli stessi caratteri unici
    let mut keys1: Vec<_> = m1.keys().collect();
    let mut keys2: Vec<_> = m2.keys().collect();
    keys1.sort();
    keys2.sort();
    if keys1 != keys2 {
        return false;
    }

    let mut counts1: Vec<_> = m1.values().collect();
    let mut counts2: Vec<_> = m2.values().collect();
    counts1.sort();
    counts2.sort();

    counts1 == counts2
}

#[cfg(test)]
mod tests {
    use super::determine_if_two_strings_are_close;

    #[test]
    fn test_determine_if_two_string_are_close(){
        let word1 = String::from("abc");
        let word2 = String::from("bca");
        let result = determine_if_two_strings_are_close(&word1, &word2);
        assert_eq!(result, true)
    }

    #[test]
    fn test_false_determine_if_two_string_are_close(){
        let word1 = String::from("a");
        let word2 = String::from("aa");
        let result = determine_if_two_strings_are_close(&word1, &word2);
        assert_eq!(result, false)
    }
}