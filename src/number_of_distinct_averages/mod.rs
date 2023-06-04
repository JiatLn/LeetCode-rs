use std::collections::HashSet;

/// <https://leetcode.cn/problems/number-of-distinct-averages/>
struct Solution;

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        let mut hash_set: HashSet<i32> = HashSet::new();
        nums.sort();
        let len = nums.len();
        for i in 0..len / 2 {
            hash_set.insert(nums[i] + nums[len - 1 - i]);
        }
        hash_set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::number_of_distinct_averages::Solution;

    #[test]
    fn test_distinct_averages() {
        let res = Solution::distinct_averages(vec![4, 1, 4, 0, 3, 5]);
        assert_eq!(res, 2);

        let res = Solution::distinct_averages(vec![1, 100]);
        assert_eq!(res, 1);
    }
}
