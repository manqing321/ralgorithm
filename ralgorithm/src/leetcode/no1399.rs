use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut mp = HashMap::<i32, i32>::new();
        let mut dp = vec![0];
        let mut ans = 0;
        let mut max_cnt = 0;
        for _ in 0..n {
            let mut inc = true;
            for idx in 0..dp.len() {
                dp[idx] += 1;
                if dp[idx] == 10 {
                    dp[idx] = 0;
                    continue;
                }
                inc = false;
                break;
            }
            if inc {
                dp.push(1);
            }
            let sum_bit = dp.iter().sum();
            *mp.entry(sum_bit).or_default() += 1;
            if mp[&sum_bit] > max_cnt {
                max_cnt = mp[&sum_bit];
                ans = 1;
            } else if mp[&sum_bit] == max_cnt {
                ans += 1;
            }
        }
        ans
    }
}
