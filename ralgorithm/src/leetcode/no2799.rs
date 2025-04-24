use std::collections::{HashMap, HashSet};

pub struct Solution;
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let cnt = nums.iter().collect::<HashSet<_>>().len();
        let mut mp = HashMap::<i32, i32>::new();
        let mut left = 0;
        let mut right = 0;
        while left < nums.len() {
            while right < nums.len() && mp.len() != cnt {
                *mp.entry(nums[right]).or_default() += 1;
                right += 1;
            }
            if mp.len() == cnt {
                ans += nums.len() - right + 1;
            }
            if let Some(v) = mp.get_mut(&nums[left]) {
                if *v > 1 {
                    *v -= 1;
                } else {
                    mp.remove(&nums[left]);
                }
            }
            left += 1;
        }
        return ans as i32;
    }
}
