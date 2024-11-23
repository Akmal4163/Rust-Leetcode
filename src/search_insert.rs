pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    
    let index = nums.iter().position(|&x| x == target);
    let mut result = nums.clone();

    match index {
        Some(i) => return i as i32,
        None => {
            result.push(target);
            result.sort();
            let index = result.iter().position(|&x| x == target);
            match index {
                Some(j) => return j as i32,
                None => return 0,
            }

        }
    }

}