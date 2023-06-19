/// <https://leetcode.cn/problems/greatest-sum-divisible-by-three/>
struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut ans = vec![0, 0, 0];
        nums.iter().for_each(|&num| {
            let a = ans[0] + num;
            let b = ans[1] + num;
            let c = ans[2] + num;
            ans[(a % 3) as usize] = a.max(ans[(a % 3) as usize]);
            ans[(b % 3) as usize] = b.max(ans[(b % 3) as usize]);
            ans[(c % 3) as usize] = c.max(ans[(c % 3) as usize]);
        });
        ans[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::greatest_sum_divisible_by_three::Solution;

    #[test]
    fn test_() {
        let ans = Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]);
        assert_eq!(ans, 18);

        let ans = Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]);
        assert_eq!(ans, 12);
    }
}
