pub fn longest_common_prefix(input: Vec<String>) -> String {
    
    if input.is_empty() {
        return String::new();
    }
    
    let mut prefix_word = input[0].clone();
    for words in &input[1..] {
        while !words.starts_with(&prefix_word) {
            prefix_word.pop();
            if prefix_word.is_empty() {
                return String::new();
            }
        }
    }

    return prefix_word;
} 