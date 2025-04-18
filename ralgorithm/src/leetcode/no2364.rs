
pub struct Solution;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let len = nums.len();
        let mut vec = nums
            .iter()
            .enumerate()
            .map(|(idx, &num)| num - idx as i32)
            .collect::<Vec<i32>>();
        vec.sort();
        let mut idx = 0;
        let mut cnt = 0;
        while idx < len {
            let val = vec[idx];
            let mut fast = idx + 1;
            while fast < len && vec[fast] == val {
                cnt += fast - idx;
                fast += 1;
            }
            idx = fast;
        }
        (((len * (len - 1)) >> 1) - cnt) as i64
    }
}
