pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let k = k as usize;
        let mut mp: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            if let Some(seq) = mp.get(&num) {
                ans += seq.iter().filter(|x| *x * idx % k == 0).count();
            }
            mp.entry(num).or_default().push(idx);
        }
        return ans as i32;
    }
}
