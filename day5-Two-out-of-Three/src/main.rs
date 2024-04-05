use std::collections::HashMap;

fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    fn count_occurrences(nums: &[i32]) -> HashMap<i32, bool> {
        let mut counts = HashMap::new();
        for &num in nums {
            counts.insert(num, true);
        }
        counts
    }

    let counts1 = count_occurrences(&nums1);
    let counts2 = count_occurrences(&nums2);
    let counts3 = count_occurrences(&nums3);

    let mut appearance_count = HashMap::new();

    fn increment_appearance_count(num: i32, counts: &HashMap<i32, bool>, appearance_count: &mut HashMap<i32, i32>) {
        if counts.contains_key(&num) {
            *appearance_count.entry(num).or_insert(0) += 1;
        }
    }

    for (&num, _) in &counts1 {
        increment_appearance_count(num, &counts2, &mut appearance_count);
        increment_appearance_count(num, &counts3, &mut appearance_count);
    }
    for (&num, _) in &counts2 {
        increment_appearance_count(num, &counts1, &mut appearance_count);
        increment_appearance_count(num, &counts3, &mut appearance_count);
    }
    for (&num, _) in &counts3 {
        increment_appearance_count(num, &counts1, &mut appearance_count);
        increment_appearance_count(num, &counts2, &mut appearance_count);
    }

    let mut result = vec![];
    for (&num, &count) in &appearance_count {
        if count >= 2 {
            result.push(num);
        }
    }

    result
}

fn main() {
    let nums1 = vec![1, 1, 3, 2];
    let nums2 = vec![2, 3];
    let nums3 = vec![3];

    let result = two_out_of_three(nums1, nums2, nums3);
    println!("Numbers appearing in at least two out of three vectors: {:?}", result);
}
