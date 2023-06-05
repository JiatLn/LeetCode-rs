/// <https://leetcode.cn/problems/apply-operations-to-an-array/submissions/>
struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        for i in 0..len - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] = nums[i] * 2;
                nums[i + 1] = 0;
            }
        }
        nums = nums
            .iter()
            .filter(|&x| *x != 0)
            .map(|&x| x)
            .collect::<Vec<i32>>();
        nums.resize(len, 0);
        nums
    }
}

#[cfg(test)]
mod tests {
    use crate::apply_operations_to_an_array::Solution;

    #[test]
    fn test_apply_operations() {
        let ans = Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]);
        assert_eq!(ans, vec![1, 4, 2, 0, 0, 0]);

        let ans = Solution::apply_operations(vec![0, 1]);
        assert_eq!(ans, vec![1, 0]);
    }
}
