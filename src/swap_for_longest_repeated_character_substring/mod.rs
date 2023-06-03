use std::collections::HashMap;

/// <https://leetcode.cn/problems/swap-for-longest-repeated-character-substring/>
pub struct Solution;

impl Solution {
    /// 滑动窗口
    pub fn max_rep_opt1(text: String) -> i32 {
        let chs = text.as_bytes();
        let mut hm = HashMap::new();
        for &ch in chs.iter() {
            match hm.get(&ch) {
                Some(v) => {
                    hm.insert(ch, v + 1);
                }
                None => {
                    hm.insert(ch, 1);
                }
            }
        }
        let n = chs.len();
        let mut i = 0;
        let mut ans = 0;
        while i < n {
            let mut j = i;
            while j < n && chs[j] == chs[i] {
                j += 1;
            }
            let mut k = j + 1;
            while k < n && chs[k] == chs[i] {
                k += 1;
            }
            ans = ans.max(
                *hm
                    .get(&chs[i])
                    .unwrap()
                    .min(&(k - i))
            );
            i = j;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_rep_opt1() {
        let res = Solution::max_rep_opt1("ababa".to_string());
        assert_eq!(res, 3);

        let res = Solution::max_rep_opt1("aaabaaa".to_string());
        assert_eq!(res, 6);

        let res = Solution::max_rep_opt1("aaabbaaa".to_string());
        assert_eq!(res, 4);

        let res = Solution::max_rep_opt1("aaaaa".to_string());
        assert_eq!(res, 5);

        let res = Solution::max_rep_opt1("abcdef".to_string());
        assert_eq!(res, 1);
    }
}
