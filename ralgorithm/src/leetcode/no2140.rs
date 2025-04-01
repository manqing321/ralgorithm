pub struct Solution;
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut lst = vec![0; questions.len() + 1];
        for idx in (0..questions.len()).rev() {
            let unselect = lst[idx + 1];
            let mut select = questions[idx][0] as i64;
            let rear_idx = idx + questions[idx][1] as usize + 1;
            if rear_idx < lst.len() {
                select += lst[rear_idx]
            }
            lst[idx] = unselect.max(select)
        }
        return lst[0];
    }
}
