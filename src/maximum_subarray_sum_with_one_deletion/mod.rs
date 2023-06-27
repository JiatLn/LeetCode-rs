/// <https://leetcode.cn/problems/maximum-subarray-sum-with-one-deletion/>
struct Solution;

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        // dp0 删除0个 dp1 删除1个
        let mut dp0 = arr[0];
        let mut dp1 = 0;
        arr.iter()
            .skip(1)
            .fold(arr[0], |acc, &x| {
                dp1 = dp0.max(dp1 + x); // 删除当前 即 dp0 或 删除之前的 保留当前的x 及 dp1 + x
                dp0 = dp0.max(0) + x; // 如果 dp0 小于 0 则抛弃前面的 此时 只有 x 否则 加上 dp0
                acc.max(dp0).max(dp1) // 计算局部最大值
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::maximum_subarray_sum_with_one_deletion::Solution;

    #[test]
    fn test_maximum_sum() {
        let ans = Solution::maximum_sum(vec![1, -2, 0, 3]);
        assert_eq!(ans, 4);

        let ans = Solution::maximum_sum(vec![1, -2, -2, 3]);
        assert_eq!(ans, 3);

        let ans = Solution::maximum_sum(vec![-1, -1, -1, -1]);
        assert_eq!(ans, -1);
    }
}
