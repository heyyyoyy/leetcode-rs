use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(HashMap::<[u8; 26], Vec<String>>::new(), |mut acc, s| {
                let arr = s.chars().fold([0; 26], |mut a, ch| {
                    a[ch as usize - 'a' as usize] += 1;
                    a
                });

                acc.entry(arr)
                    .and_modify(|a| a.push(s.clone()))
                    .or_insert(vec![s]);
                acc
            })
            .into_values()
            .collect()
    }
}
