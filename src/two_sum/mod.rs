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
    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut tuple_nums = nums.iter().enumerate().collect::<Vec<(_, _)>>();
        tuple_nums.sort_by(|a, b| a.1.cmp(b.1));
        let mut left = 0;
        let mut right = tuple_nums.len() - 1;
        while left < right {
            match (tuple_nums[left].1 + tuple_nums[right].1).cmp(&target) {
                std::cmp::Ordering::Less => {
                    left += 1;
                }
                std::cmp::Ordering::Equal => {
                    return vec![tuple_nums[left].0 as i32, tuple_nums[right].0 as i32];
                }
                std::cmp::Ordering::Greater => {
                    right -= 1;
                }
            }
        }
        vec![]
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

    #[test]
    fn test_two_sum_2() {
        let nums = vec![2, 7, 11, 15];
        let ans = Solution::two_sum_2(nums, 9);
        assert_eq!(ans, vec![0, 1]);

        let nums = vec![3, 2, 4];
        let ans = Solution::two_sum_2(nums, 6);
        assert_eq!(ans, vec![1, 2]);
    }
}
