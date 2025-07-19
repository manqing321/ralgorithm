use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let st: HashSet<String> = folder.iter().cloned().collect();
        let mut ans = Vec::new();
        for s in folder.iter() {
            let mut flag = false;
            for (idx, ch) in s.chars().enumerate() {
                if ch == '/' {
                    let pre = &s[0..idx];
                    if st.contains(pre) {
                        flag = true;
                        break;
                    }
                }
            }
            if !flag {
                ans.push(s.clone());
            }
        }
        return ans;
    }
}
