struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut balance: i32 = 0;
        let mut count: i32 = 0;

        
        for letter in s.chars() {
            if letter == 'R' {
                balance += 1;
            } else if letter == 'L' {
                balance -= 1;
            }
            
            if balance == 0 {
                count += 1;
            }
        }

        count
    }
}

fn main() {
    let input: String = String::from("RRLL"); // Corrected: Use String::from to create a String
    let output: i32 = Solution::balanced_string_split(input);
    println!("{}", output); // Output: 1
}