use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut st = HashSet::new();
        for i in (0..nums.len()).rev() {
            if st.contains(&nums[i]) {
                return (i / 3 + 1) as i32;
            }
            st.insert(nums[i]);
        }
        0
    }
}
