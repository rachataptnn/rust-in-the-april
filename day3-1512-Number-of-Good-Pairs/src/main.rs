fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut freq = [0; 101];

    for &num in nums.iter() {
        freq[num as usize] += 1;
    }

    for &f in freq.iter() {
        if f > 1 {
            count += (f * (f - 1)) / 2;
        }
    }

    count
}

fn main() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    let pairs = num_identical_pairs(nums);
    println!("Number of good pairs: {}", pairs);
}
