use std::collections::HashSet;

pub struct Solution;

impl Solution {
    fn verify(source: &Vec<char>, skips: &Vec<usize>) -> bool {
        let mut l_cnt = 0;
        let mut r_cnt = 0;
        for l_idx in 0..source.len() {
            let r_idx = source.len() - 1 - l_idx;
            if !skips.contains(&l_idx) {
                if source[l_idx] == '(' {
                    l_cnt += 1;
                }
                if source[l_idx] == ')' {
                    l_cnt -= 1;
                }
                if l_cnt < 0 {
                    return false;
                }
            }
            if !skips.contains(&r_idx) {
                if source[r_idx] == '(' {
                    r_cnt -= 1;
                }
                if source[r_idx] == ')' {
                    r_cnt += 1;
                }
                if r_cnt < 0 {
                    return false;
                }
            } 
        }
        return true;
    }
    
    fn dfs(source: &Vec<char>, ans: &mut HashSet<String>, selected: &mut Vec<usize>, idx: usize, left_limit: usize, right_limit: usize, min_len: &mut usize) {
        if idx == source.len() || (left_limit == 0 && right_limit == 0) {
            if Self::verify(source, selected) {
                let s: String = source.iter().enumerate().filter(|(i,_)| !selected.contains(i)).map(|(_, c)|c).collect();
                if s.len() < *min_len {
                    ans.clear();
                }
                *min_len = s.len();
                ans.insert(s);
            }
            return;
        }

        // not take it
        Self::dfs(source, ans, selected, idx + 1, left_limit, right_limit, min_len);

        // take it
        if source[idx] == '(' && left_limit > 0 {
            selected.push(idx);
            Self::dfs(source, ans, selected, idx + 1, left_limit - 1, right_limit, min_len);
            selected.pop();
        }
        if source[idx] == ')' && right_limit > 0 {
            selected.push(idx);
            Self::dfs(source, ans, selected, idx + 1, left_limit, right_limit - 1, min_len);
            selected.pop();
        }
    }

    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut ans = HashSet::<String>::new();
        let mut selected = Vec::<usize>::new();
        let source: Vec<char> = s.chars().collect::<Vec<char>>();
        let mut left_limit = 0;
        let mut right_limit = 0;
        let mut left_cnt = 0;
        let mut right_cnt = 0;
        for ch in source.iter() {
            if *ch == '(' {
                left_cnt += 1;
            }
            if *ch == ')' {
                left_cnt -= 1;
            }
            if left_cnt == -1 {
                left_cnt = 0;
                right_limit += 1;
            }
        }
        for ch in source.iter().rev() {
            if *ch == ')' {
                right_cnt += 1;
            }
            if *ch == '(' {
                right_cnt -= 1;
            }
            if right_cnt == -1 {
                right_cnt = 0;
                left_limit += 1;
            }
        }
        let mut min_len = s.len();
        Self::dfs(&source,&mut ans, &mut selected, 0, left_limit, right_limit, &mut min_len);
        if ans.len() > 0 {
            return ans.into_iter().collect();
        }
        return vec![String::from("")];
    }
}
