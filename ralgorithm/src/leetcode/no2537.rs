use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut mp = HashMap::new();
        let mut ans: i64 = 0;
        let mut cnt = 0;
        let mut slow = 0;
        let mut fast = 0;
        while slow < nums.len() {
            while cnt < k && fast < nums.len() {
                mp.entry(nums[fast]).and_modify(|x| *x += 1).or_insert(1);
                cnt += mp[&nums[fast]] - 1;
                fast += 1;
            }
            if cnt >= k {
                ans += (nums.len() + 1 - fast) as i64;
            }
            mp.entry(nums[slow]).and_modify(|x| *x -= 1);
            cnt -= mp[&nums[slow]];
            slow += 1;
        }
        ans
    }
}
