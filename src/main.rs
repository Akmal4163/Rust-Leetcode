fn main() {
    println!("welcome !!!");

}

#[cfg(test)]
mod tests {
    use lc01::*;

    #[test]
    pub fn lc_search_insert_1() {
        let nums =  vec![1,3,5,6];
        let actual = search_insert(nums, 5);
        let expected = 2;
        assert_eq!(actual, expected, "expected {:?} but got {:?}", expected, actual);
    }

    #[test]
    pub fn lc_search_insert_2() {
        let nums =  vec![1,3,5,6];
        let actual = search_insert(nums, 2);
        let expected = 1;
        assert_eq!(actual, expected, "expected {:?} but got {:?}", expected, actual);
    }

    #[test]
    pub fn lc_search_insert_3() {
        let nums =  vec![1,3,5,6];
        let actual = search_insert(nums, 7);
        let expected = 4;
        assert_eq!(actual, expected, "expected {:?} but got {:?}", expected, actual);
    }


}
