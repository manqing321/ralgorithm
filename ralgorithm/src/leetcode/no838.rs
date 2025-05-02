pub struct Solution;
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let size = dominoes.len();
        let mut dominoes :Vec<u8> = dominoes.bytes().collect();
        let mut l_vec = vec![0; size];
        let mut l_cnt = 0;
        for (i, c) in dominoes.iter().enumerate().rev() {
            match c {
                b'L' => l_cnt = size,
                b'R' => l_cnt = 0,
                _ => l_cnt = l_cnt.saturating_sub(1),
            }
            l_vec[i] = size - l_cnt;
        }
        l_vec.iter().for_each(|x| print!("{} ", x));
        println!();
        let mut r_cnt = 0;
        for i in 0..dominoes.len() {
            let c = dominoes[i];
            match c {
                b'R' => r_cnt = size,
                b'L' => r_cnt = 0,
                _ => r_cnt = r_cnt.saturating_sub(1),
            }
            print!("{} ",size - r_cnt);
            l_cnt = l_vec[i];
            if l_cnt != size - r_cnt {
                dominoes[i] = if l_cnt <  size - r_cnt { b'L' } else { b'R' };
            }
        }
        println!();
        String::from_utf8(dominoes).unwrap()
    }
}
