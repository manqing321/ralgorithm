pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut ans = 0;
        let mut pre:Option<i32> = None;
        let vec:Vec<i32> = s.chars().map(|c|c as i32).collect();
        for ch in  vec
        {
             if let Some(c) = pre 
             {
                let diff = c - ch;
                if diff < 0
                {
                    ans -= diff;
                }else{
                    ans += diff;
                }
             }
             pre = Some(ch)
        }
        return ans;
    }
}
