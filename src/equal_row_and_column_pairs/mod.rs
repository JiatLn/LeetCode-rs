/// <https://leetcode.cn/problems/equal-row-and-column-pairs/>
struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;
        for i in 0..n {
            let mut col = Vec::with_capacity(n);
            for j in 0..n {
                col.push(grid[j][i]);
            }
            for row in grid.iter() {
                if *row == col {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::equal_row_and_column_pairs::Solution;
    #[test]
    fn test_equal_pairs() {
        let ans = Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]);
        assert_eq!(ans, 1);

        let ans = Solution::equal_pairs(
            vec![vec![3, 1, 2, 2], vec![1, 4, 4, 5], vec![2, 4, 2, 2], vec![2, 4, 2, 2]]
        );
        assert_eq!(ans, 3);
    }
}
