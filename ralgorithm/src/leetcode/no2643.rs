pub struct Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0, 0];
        for idx in 0..mat.len() {
            let nums = &mat[idx];
            let cnt: i32 = nums.iter().sum();
            if cnt > ans[1] {
                ans[0] = idx as i32;
                ans[1] = cnt;
            }
        }
        return ans;
    }
}
