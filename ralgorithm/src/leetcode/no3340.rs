impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut sign = 1;
        let mut diff = 0;
        for ch in num.chars() {
            let mut ch = ch as i32;
            ch = ch - ('0' as i32);
            diff += sign * ch;
            sigh = -sign;
        }
        return diff == 0;
    }
}
