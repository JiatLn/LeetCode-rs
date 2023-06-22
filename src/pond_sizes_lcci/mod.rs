/// <https://leetcode.cn/problems/pond-sizes-lcci/>
struct Solution;

impl Solution {
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        let dir = vec![[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
        let mut ans = vec![];
        fn dfs(land: &mut Vec<Vec<i32>>, x: i32, y: i32, dir: &Vec<[i32; 2]>) -> i32 {
            if
                x < 0 ||
                y < 0 ||
                x >= (land.len() as i32) ||
                y >= (land[0].len() as i32) ||
                land[x as usize][y as usize] > 0
            {
                return 0;
            }
            let mut cnt = 1;
            land[x as usize][y as usize] = 1;
            dir.iter().for_each(|&[dx, dy]| {
                cnt += dfs(land, x + dx, y + dy, dir);
            });
            cnt
        }
        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if land[i][j] == 0 {
                    ans.push(dfs(&mut land, i as i32, j as i32, &dir));
                }
            }
        }
        ans.sort();
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::pond_sizes_lcci::Solution;

    #[test]
    fn test_pond_sizes() {
        let land = vec![vec![0, 2, 1, 0], vec![0, 1, 0, 1], vec![1, 1, 0, 1], vec![0, 1, 0, 1]];
        let ans = Solution::pond_sizes(land);
        assert_eq!(ans, vec![1, 2, 4])
    }
}
