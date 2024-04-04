fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let mut jewel_count = 0;
    for stone in s.chars() {
        if j.contains(stone) {
            jewel_count += 1;
        }
    }
    jewel_count
}

fn main() {
    let jewels = String::from("aA");
    let stones = String::from("aAAbbbb");
    let result = num_jewels_in_stones(jewels, stones);
    println!("Number of Jewels in Stones: {}", result);
}
