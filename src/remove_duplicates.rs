use std::collections::HashSet;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut temporary: HashSet<i32> = HashSet::new();
    
    for i in nums.iter() {
        temporary.insert(*i);
    }

    nums.clear();
    nums.extend(temporary.into_iter());
    nums.reverse();

    return nums.len() as i32;
}