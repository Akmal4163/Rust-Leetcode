use std::vec;

pub fn is_valid_parenthesis(input: String) -> bool {
    let mut temporary = Vec::new();

    for i in input.chars() {
        match i {
            '(' | '[' | '{' => temporary.push(i),
            ')' => {
                if temporary.pop() != Some('(') {
                    return false;
                }
            },
            ']' => {
                if temporary.pop() != Some('[') {
                    return false;
                }
            },
            '}' => {
                if temporary.pop() != Some('{') {
                    return false;
                }
            }
            _ => {},
        }
    }

    if temporary.is_empty() {
        return true;
    } else {
        return false;
    }
}