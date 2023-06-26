/// <https://leetcode.cn/problems/find-the-pivot-integer/>
struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        // if n == 1 {
        //     return 1;
        // }
        // for i in 1..n {
        //     let left = (1..=i).fold(0, |a, b| a + b);
        //     let right = (i..=n).fold(0, |a, b| a + b);
        //     if left == right {
        //         return i;
        //     }
        // }
        // -1
        let y = (((n * (n + 1)) as f64) / 2.0).floor();
        let x = y.sqrt().floor();
        if x * x == y {
            x as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::find_the_pivot_integer::Solution;

    #[test]
    fn test_pivot_integer() {
        let ans = Solution::pivot_integer(8);
        assert_eq!(ans, 6);

        let ans = Solution::pivot_integer(1);
        assert_eq!(ans, 1);
    }
}
