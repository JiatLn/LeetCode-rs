/// <https://leetcode.cn/problems/sum-in-a-matrix/>
struct Solution;

impl Solution {
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        nums.iter_mut().for_each(|row| row.sort_by(|a, b| b.cmp(&a)));
        (0..nums[0].len()).fold(0, |acc, col_idx| {
            acc +
                (1..nums.len()).fold(nums[0][col_idx], |max_val, row_idx|
                    max_val.max(nums[row_idx][col_idx])
                )
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::sum_in_a_matrix::Solution;

    #[test]
    fn test_matrix_sum() {
        let ans = Solution::matrix_sum(
            vec![vec![7, 2, 1], vec![6, 4, 2], vec![6, 5, 3], vec![3, 2, 1]]
        );
        assert_eq!(ans, 15);

        let ans = Solution::matrix_sum(
            vec![
                vec![1, 8, 16, 15, 12, 9, 15, 11, 18, 6, 16, 4, 9, 4],
                vec![3, 19, 8, 17, 19, 4, 9, 3, 2, 10, 15, 17, 3, 11],
                vec![13, 10, 19, 20, 6, 17, 15, 14, 16, 8, 1, 17, 0, 2],
                vec![12, 20, 0, 19, 15, 10, 7, 10, 2, 6, 18, 7, 7, 4],
                vec![17, 14, 2, 2, 10, 16, 15, 3, 9, 17, 9, 3, 17, 10],
                vec![17, 6, 19, 17, 18, 9, 14, 2, 19, 12, 10, 18, 7, 9],
                vec![5, 6, 5, 1, 19, 8, 15, 2, 2, 4, 4, 1, 2, 17],
                vec![12, 16, 8, 16, 7, 6, 18, 13, 18, 8, 14, 15, 20, 11],
                vec![2, 10, 19, 3, 15, 18, 20, 10, 6, 7, 0, 8, 3, 7],
                vec![11, 5, 10, 13, 1, 3, 4, 7, 1, 18, 20, 17, 19, 2],
                vec![0, 3, 20, 6, 19, 18, 3, 12, 2, 11, 3, 1, 19, 0],
                vec![6, 5, 3, 15, 6, 1, 0, 17, 13, 19, 3, 8, 2, 7],
                vec![2, 20, 9, 11, 13, 5, 1, 16, 14, 1, 19, 3, 12, 6]
            ]
        );
        assert_eq!(ans, 190);
    }
}
