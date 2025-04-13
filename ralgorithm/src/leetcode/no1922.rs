pub struct Solution;
impl Solution {
    pub fn fast_pow(base: i64, power_cnt: usize, limit: i64) -> i64 {
        if power_cnt == 0 {
            return 1;
        }
        if power_cnt % 2 == 1 {
            return base * Solution::fast_pow(base, power_cnt - 1, limit) % limit;
        }
        Solution::fast_pow(base * base % limit, power_cnt >> 1, limit)
    }
    pub fn count_good_numbers(n: i64) -> i32 {
        let odd_cnt = n >> 1;
        let limit = 10_0000_0000 + 7;
        let mut ans = Solution::fast_pow(20, odd_cnt as usize, limit);
        if n & 1 == 1 {
            ans *= 5;
            ans %= limit;
        }
        return ans as i32;
    }
}
