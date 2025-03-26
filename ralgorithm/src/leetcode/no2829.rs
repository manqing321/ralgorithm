pub struct Solution;

impl Solution {
    fn ssum(first_ele: i32, last_ele: i32, cnt: i32) -> i32 {
        ((first_ele + last_ele) * cnt) / 2
    }

    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        if n <= k / 2 {
            Solution::ssum(1, n, n)
        } else {
            Solution::ssum(1, k / 2, k / 2) + Solution::ssum(k, k + (n - k / 2) - 1, n - k / 2)
        }
    }
}
