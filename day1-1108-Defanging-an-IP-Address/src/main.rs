// Define the Solution struct
struct Solution;

// Implement the Solution struct
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}

// Main function to test the defang_i_paddr function
fn main() {
    let address = String::from("1.2.3.4");
    let defanged_address = Solution::defang_i_paddr(address);
    println!("{}", defanged_address);
}