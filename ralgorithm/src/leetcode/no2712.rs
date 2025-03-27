
pub struct Solution;

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        s.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
        .filter(|(_,v)| v[0] != v[1])
        .map(|(x, _)| (x + 1).min(s.len() - x - 1) as i64)
        .sum()
    }
}