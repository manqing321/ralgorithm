pub struct Solution;
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .iter()
            .enumerate()
            .filter(|(_, s)| s.contains(x))
            .map(|(idx, _)| idx as i32)
            .collect()
    }
}
