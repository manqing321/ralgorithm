pub struct Solution;
impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut sum: i64 = 0;
        let k = k as i64;
        let mut len = 0;
        for (idx, ch) in s.chars().rev().enumerate() {
            if ch == '1' {
                if idx > 31 {
                    continue;
                }
                let inc = 2_i64.pow(idx as u32);
                if sum + inc > k {
                    continue;
                }
                sum += inc;
            }
            len += 1;
        }
        len
    }
}
