fn is_palindrome(s: &str) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let len = s.len();
    
    for i in 0..len / 2 {
        if s[i] != s[len - 1 - i] {
            return false;
        }
    }
    
    true
}

fn main() {
    let input = "racecar"; // Change this to test different strings
    
    if is_palindrome(input) {
        println!("'{}' is a palindrome.", input);
    } else {
        println!("'{}' is not a palindrome.", input);
    }
}
