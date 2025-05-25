use std::{cmp::min, collections::HashMap};

pub struct Solution;
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut mp: HashMap<String, i32> = HashMap::new();
        let mut is_odd = false;
        for s in words.iter() {
            *mp.entry(s.clone()).or_default() += 1;
        }
        for (s, cnt) in mp.iter() {
            let rev_s = format!("{}{}", s.chars().nth(1).unwrap(), s.chars().nth(0).unwrap());
            if *s == rev_s {
                if !is_odd && cnt % 2 == 1 {
                    is_odd = true;
                }
                ans += cnt / 2 * 4;
            } else if *s > rev_s {
                let m = min(cnt, mp.get(&rev_s).unwrap_or(&0));
                ans += m * 4;
            }
        }
        if is_odd {
            ans += 2;
        }
        return ans;
    }
}
