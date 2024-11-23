pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();

    for (i, &num) in nums.iter().enumerate() {
        let remainder = target - num;
        if let Some(n) = nums.iter().enumerate().find_map(|(j, &x)| 
            if x == remainder && j > i {
                Some(j)
            } else {
                None
            }
        ) {
            result.push(i as i32);
            result.push(n as i32);
            break;
        }
    }


    return result;
}