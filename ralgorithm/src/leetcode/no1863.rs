pub struct Solution;
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let end = 1 << len;
        let mut ans = 0;
        for beg in 1..end {
            let mut idx = 0;
            let mut flag = beg;
            let mut not_or_sum = 0;
            while flag != 0 {
                if flag & 1 == 1 {
                    not_or_sum ^= nums[idx];
                }
                idx += 1;
                flag >>= 1;
            }
            ans += not_or_sum;
        }
        ans
    }
}
