pub struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut diff = 0;
        for ch in s.chars() {
            if ch == '[' {
                diff += 1;
            } else if diff > 0 {
                diff -= 1;
            }
        }
        return (diff + 1) / 2;
    }
}
