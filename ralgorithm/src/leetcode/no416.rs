pub struct Solution;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        // 奇数无法拆分
        if sum & 1 == 1 {
            return false;
        }
        // 最大值大于 half 无法拆分
        let half = sum >> 1;
        let m = nums.iter().max();
        if let Some(m) = m {
            if *m > half {
                return false;
            }
        }

        let mut dp = vec![vec![false; half as usize + 1]; nums.len()];
        for r in 0..nums.len() {
            let num = nums[r] as usize;
            if r == 0 {
                dp[r][num] = true;
                continue;
            }
            for c in 1..half as usize + 1 {
                if num == c {
                    dp[r][c] = true;
                } else if num > c {
                    dp[r][c] = dp[r - 1][c];
                } else {
                    dp[r][c] = dp[r - 1][c] || dp[r - 1][c - num];
                }
                print!("{} ", dp[r][c])
            }
            println!("")
        }

        if let Some(lst) = dp.last() {
            if let Some(lst) = lst.last() {
                return *lst;
            }
        }
        return false;
    }
}
