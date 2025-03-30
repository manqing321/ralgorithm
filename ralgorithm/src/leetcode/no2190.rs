pub struct Solution;
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ans = String::new();
        for (idx, &num) in spaces.iter().enumerate() {
            if idx == 0 {
                ans.push_str(&s[0..num as usize]);
            } else {
                ans.push_str(&s[spaces[idx - 1] as usize..num as usize]);
            }
            ans.push(' ');
            if idx == spaces.len() - 1 {
                ans.push_str(&s[num as usize..]);
            }
        }
        return ans;
    }
}
