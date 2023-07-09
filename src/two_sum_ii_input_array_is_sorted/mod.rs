/// <https://leetcode.cn/problems/two-sum-ii-input-array-is-sorted/>
struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        loop {
            if numbers[left] + numbers[right] == target {
                return vec![(left as i32) + 1, (right as i32) + 1];
            }
            while numbers[left] + numbers[right] < target {
                left += 1;
            }
            while numbers[left] + numbers[right] > target {
                right -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::two_sum_ii_input_array_is_sorted::Solution;

    #[test]
    fn test_two_sum() {
        let ans = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(ans, vec![1, 2]);

        let ans = Solution::two_sum(vec![2, 3, 4], 6);
        assert_eq!(ans, vec![1, 3]);
    }
}
