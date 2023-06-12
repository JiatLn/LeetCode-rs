use std::collections::HashMap;

/// <https://leetcode.cn/problems/two-sum/>
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::with_capacity(nums.len() / 2);
        for (i, &num) in nums.iter().enumerate() {
            match hm.get(&(target - num)) {
                Some(&val) => {
                    return vec![val, i as i32];
                }
                None => {
                    hm.insert(num, i as i32);
                }
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use crate::two_sum::Solution;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let ans = Solution::two_sum(nums, 9);
        assert_eq!(ans, vec![0, 1]);

        let nums = vec![3, 2, 4];
        let ans = Solution::two_sum(nums, 6);
        assert_eq!(ans, vec![1, 2]);
    }
}
