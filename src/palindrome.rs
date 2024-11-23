pub fn is_palindrome(num: i32) -> bool {
    let original = num.to_string();
    let reversed = num.to_string().chars().rev().collect::<String>();

    if original == reversed {
        true
    } else {
        false
    }
}