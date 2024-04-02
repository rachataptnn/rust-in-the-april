struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut concatenated = nums.clone();
        concatenated.extend(nums.iter());
        concatenated
    }
}

fn main() {
    let nums = vec![1, 3, 2, 1];
    let concatenated = Solution::get_concatenation(nums);
    println!("Concatenated integers: {:?}", concatenated);
}
