use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut mp: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            mp.entry(num).and_modify(|num| *num += 1).or_insert(1);
        }
        while !mp.is_empty() {
            let mut vector: Vec<i32> = Vec::new();
            for num in mp.keys() {
                vector.push(*num);
            }
            for num in vector.iter() {
                mp.entry(*num).and_modify(|num| *num -= 1);
                if mp[num] <= 0 {
                    mp.remove(num);
                }
            }
            ans.push(vector);
        }
        return ans;
    }
}
