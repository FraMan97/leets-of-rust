// Given an array of integers arr, return true if the number of occurrences of each 
// value in the array is unique or false otherwise.
use std::collections::{HashMap, HashSet};

pub fn unique_number_occurrences(arr: &[i64]) -> bool {
    let mut occurrences: HashMap<i64, usize> = HashMap::new();
    for &el in arr {
        *occurrences.entry(el).or_insert(0) += 1;
    }

    let mut seen_counts = HashSet::new();
    
    for &count in occurrences.values() {
        if !seen_counts.insert(count) {
            return false;
        }
    }

    true
}


#[cfg(test)]
mod tests {
    use super::unique_number_occurrences;

    #[test]
    fn test_unique_number_occurrences(){
        let arr = [1,2,2,1,1,3];
        let result = unique_number_occurrences(&arr);
        assert_eq!(result, true);
    }

    #[test]
    fn test_false_unique_number_occurrences(){
        let arr = [1,2];
        let result = unique_number_occurrences(&arr);
        assert_eq!(result, false);
    }

    #[test]
    fn test_empty_unique_number_occurrences(){
        let arr = [];
        let result = unique_number_occurrences(&arr);
        assert_eq!(result, true);
    }
}