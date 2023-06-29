/// <https://leetcode.cn/problems/reconstruct-a-2-row-binary-matrix/>
struct Solution;

impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let sum = colsum.iter().sum::<i32>();
        if upper + lower != sum {
            return vec![];
        }
        let mut ans = vec![Vec::with_capacity(colsum.len()), Vec::with_capacity(colsum.len())];
        ans[0].resize(colsum.len(), 0);
        ans[1].resize(colsum.len(), 0);
        colsum
            .iter()
            .enumerate()
            .for_each(|(i, &sum)| {
                match sum {
                    2 => {
                        upper -= 1;
                        lower -= 1;
                        ans[0][i] = 1;
                        ans[1][i] = 1;
                    }
                    1 => {
                        if upper >= lower {
                            ans[0][i] = 1;
                            upper -= 1;
                        } else {
                            ans[1][i] = 1;
                            lower -= 1;
                        }
                    }
                    _ => (),
                }
            });
        if upper > 0 || lower > 0 {
            vec![]
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reconstruct_a_2_row_binary_matrix::Solution;

    #[test]
    fn test_reconstruct_matrix() {
        let ans = Solution::reconstruct_matrix(2, 1, vec![1, 1, 1]);
        assert_eq!(ans, vec![vec![1, 1, 0], vec![0, 0, 1]]);

        let ans = Solution::reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]);
        assert_eq!(
            ans,
            vec![vec![1, 1, 1, 0, 0, 0, 1, 1, 0, 0], vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1]]
        );

        let ans = Solution::reconstruct_matrix(2, 3, vec![2, 2, 1, 1]);
        assert_eq!(ans, vec![] as Vec<Vec<i32>>);

        let ans = Solution::reconstruct_matrix(9, 2, vec![0, 1, 2, 0, 0, 0, 0, 0, 2, 1, 2, 1, 2]);
        assert_eq!(ans, vec![] as Vec<Vec<i32>>);
    }
}
