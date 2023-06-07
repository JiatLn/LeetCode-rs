/// <https://leetcode.cn/problems/mice-and-cheese/>
struct Solution;

impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let len = reward1.len();
        let mut diffs = Vec::with_capacity(len);
        for i in 0..len {
            diffs.push([reward1[i] - reward2[i], reward1[i], reward2[i]]);
        }
        diffs.sort_by(|a, b| b[0].cmp(&a[0]));
        let mut cnt = 0;
        diffs.iter().fold(0, |acc, e| {
            acc +
                (if cnt < k {
                    cnt += 1;
                    e[1]
                } else {
                    e[2]
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mice_and_cheese::Solution;

    #[test]
    fn test_mice_and_cheese() {
        let reward1 = vec![1, 1, 3, 4];
        let reward2 = vec![4, 4, 1, 1];
        let k = 2;
        let ans = Solution::mice_and_cheese(reward1, reward2, k);
        assert_eq!(ans, 15);

        let reward1 = vec![1, 1];
        let reward2 = vec![1, 1];
        let k = 2;
        let ans = Solution::mice_and_cheese(reward1, reward2, k);
        assert_eq!(ans, 2);
    }
}
