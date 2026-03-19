// You are given a string s, which contains stars *.

// In one operation, you can:

//     Choose a star in s.
//     Remove the closest non-star character to its left, as well as remove the star itself.

// Return the string after all stars have been removed.

// Note:

//     The input will be generated such that the operation is always possible.
//     It can be shown that the resulting string will always be unique.
pub fn removing_stars_from_string(s: &String) -> String {
    let mut stack = String::new();

    for c in s.chars() {
        if c == '*' {
            stack.pop(); 
        } else {
            stack.push(c);
        }
    }

    stack
}


#[cfg(test)]
mod tests {
    use super::removing_stars_from_string;

    #[test]
    fn test_removing_stars_from_string(){
        let s: String = String::from("leet**cod*e");
        let result = removing_stars_from_string(&s);
        assert_eq!(result, "lecoe");
    }

    #[test]
    fn test_empty_removing_stars_from_string(){
        let s: String = String::from("");
        let result = removing_stars_from_string(&s);
        assert_eq!(result, "");
    }
}