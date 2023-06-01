/// <https://leetcode.cn/problems/maximum-tastiness-of-candy-basket/>
struct Solution;

impl Solution {
    /// 贪心 二分
    pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
        price.sort();
        let mut left = 0;
        let mut right = price.last().unwrap() - price[0];
        let check = |tastiness| {
            let mut prev = -price.last().unwrap();
            let mut cnt = 0;
            for &p in price.iter() {
                if p - prev >= tastiness {
                    cnt = cnt + 1;
                    prev = p;
                }
            }
            cnt >= k
        };
        while left < right {
            let mid = (((left + right + 1) as f64) / 2.0).floor() as i32;
            if check(mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use crate::maximum_tastiness_of_candy_basket::Solution;

    #[test]
    fn test_maximum_tastiness() {
        let ans = Solution::maximum_tastiness(vec![13, 5, 1, 8, 21, 2], 3);
        assert_eq!(ans, 8);

        let ans = Solution::maximum_tastiness(vec![1, 3, 1], 2);
        assert_eq!(ans, 2);

        let ans = Solution::maximum_tastiness(vec![34, 116, 83, 15, 150, 56, 69, 42, 26], 6);
        assert_eq!(ans, 19);
    }
}
