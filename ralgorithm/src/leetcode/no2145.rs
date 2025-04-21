pub struct Solution;
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut max_num = 0;
        let mut min_num = 0;
        let mut seed: i64 = 0;
        for num in differences.iter() {
            seed += *num as i64;
            max_num = max_num.max(seed);
            min_num = min_num.min(seed);
        }
        let array_range = max_num - min_num;
        let scope = (upper - lower) as i64;

        if scope < array_range {
            0
        } else {
            (scope - array_range + 1) as i32
        }
    }
}
