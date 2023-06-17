/// <https://leetcode.cn/problems/powx-n/solution/powx-n-by-leetcode-solution/>
struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn quick_m(mut x: f64, mut n: i64) -> f64 {
            let mut ans = 1.0;
            while n > 0 {
                if (n & 1) == 1 {
                    ans *= x;
                }
                x *= x;
                n >>= 1;
            }
            ans
        }
        if n >= 0 {
            quick_m(x, n as i64)
        } else {
            1.0 / quick_m(x, -(n as i64))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::powx_n_by_leetcode_solution::Solution;

    #[test]
    fn test_my_pow() {
        let ans = Solution::my_pow(2.0, 10);
        assert_eq!(ans, 1024.0);

        let ans = Solution::my_pow(2.0, -2147483648);
        assert_eq!(ans, 0.0);

        let ans = Solution::my_pow(2.1, 3);
        assert_eq!(ans, 9.261000000000001)
    }
}
