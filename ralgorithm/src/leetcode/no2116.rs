pub struct Solution;
impl Solution {
    fn check_balance(s: &str, locked: &str, forward: bool) -> bool {
        let chaos: Vec<_> = if forward {
            s.chars().zip(locked.chars()).collect()
        } else {
            s.chars().rev().zip(locked.chars().rev()).collect()
        };
        let mut balance = 0;
        let sign = if forward { '(' } else { ')' };
        for (ch, lock) in chaos {
            if lock == '0' || ch == sign {
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance < 0 {
                return false;
            }
        }
        true
    }
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let len = s.len();
        if len % 2 == 1 {
            return false;
        }
        return Solution::check_balance(&s, &locked, true)
            && Solution::check_balance(&s, &locked, false);
    }
}
