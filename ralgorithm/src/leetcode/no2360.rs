pub struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut label = 0;
        let mut lst = vec![0; edges.len()];
        for i in 0..edges.len() {
            if lst[i] > 0 {
                continue;
            }
            let mut core_label = label;
            let mut pos = i as i32;
            while pos != -1 {
                core_label += 1;
                let idx = pos as usize;
                if lst[idx] > 0 {
                    if lst[idx] > label {
                        ans = ans.max(core_label - lst[idx])
                    }
                    break;
                }
                lst[idx] = core_label;
                pos = edges[idx];
            }
            label = core_label;
        }
        ans
    }
}
