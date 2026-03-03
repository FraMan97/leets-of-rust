// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

// A subsequence of a string is a new string that is formed from the original string by deleting 
// some (can be none) of the characters without disturbing the relative positions of the remaining characters. 
// (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

pub fn is_subsequence(s: &str, t: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    
    let mut s_iter = s.chars();
    let mut current_s_char = s_iter.next();

    for t_char in t.chars() {
        if let Some(target) = current_s_char {
            if t_char == target {
                current_s_char = s_iter.next();
            }
        }
        
        // Se current_s_char è None, abbiamo trovato tutti i caratteri di s
        if current_s_char.is_none() {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::is_subsequence;

    #[test]
    fn test_true_is_subsequence(){
        let s = "abc";
        let t = "ahbgdc";
        let result = is_subsequence(&s, &t);
        assert_eq!(result, true);
    }

    #[test]
    fn test_false_is_subsequence(){
        let s = "axc";
        let t = "ahbgdc";
        let result = is_subsequence(&s, &t);
        assert_eq!(result, false);
    }


    #[test]
    fn test_empty_is_subsequence(){
        let s = "";
        let t = "ahbgdc";
        let result = is_subsequence(&s, &t);
        assert_eq!(result, true);
    }

}
