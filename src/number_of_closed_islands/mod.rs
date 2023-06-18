/// <https://leetcode.cn/problems/number-of-closed-islands/>
struct Solution;

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let col = grid[0].len();
        let mut ans = 0;
        for i in 1..row - 1 {
            for j in 1..col - 1 {
                if
                    grid[i][j] == 0 &&
                    Self::dfs(&mut grid, i as i32, j as i32, row as i32, col as i32)
                {
                    ans += 1;
                }
            }
        }
        ans
    }
    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, row: i32, col: i32) -> bool {
        if i < 0 || i >= row || j < 0 || j >= col {
            return false;
        }
        if grid[i as usize][j as usize] == 1 {
            return true;
        }
        grid[i as usize][j as usize] = 1;
        let b1 = Self::dfs(grid, i + 1, j, row, col);
        let b2 = Self::dfs(grid, i - 1, j, row, col);
        let b3 = Self::dfs(grid, i, j + 1, row, col);
        let b4 = Self::dfs(grid, i, j - 1, row, col);
        b1 && b2 && b3 && b4
    }
}

#[cfg(test)]
mod tests {
    use crate::number_of_closed_islands::Solution;

    #[test]
    fn test_closed_island() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0]
        ];
        let ans = Solution::closed_island(grid);
        assert_eq!(ans, 2);

        let grid = vec![vec![0, 0, 1, 0, 0], vec![0, 1, 0, 1, 0], vec![0, 1, 1, 1, 0]];
        let ans = Solution::closed_island(grid);
        assert_eq!(ans, 1);
    }
}
