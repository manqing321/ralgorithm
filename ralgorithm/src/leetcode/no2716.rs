use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        s.chars().collect::<HashSet<_>>().len() as i32
    }
}