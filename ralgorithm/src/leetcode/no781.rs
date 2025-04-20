use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut mp = HashMap::<i32, i32>::new();
        for num in answers.iter().map(|x| *x + 1) {
            let iter = mp.entry(num).or_default();
            if *iter == num {
                *iter = 0;
                ans += num;
            }
            *iter += 1;
        }
        ans + mp
            .iter()
            .map(|(key, _)| key)
            .fold(0, |seed, num| seed + num)
    }
}
