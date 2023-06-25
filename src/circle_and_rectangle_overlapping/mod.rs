/// <https://leetcode.cn/problems/circle-and-rectangle-overlapping/>
struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32
    ) -> bool {
        let dx = (0).max(x1 - x_center).max(x_center - x2);
        let dy = (0).max(y1 - y_center).max(y_center - y2);
        dx * dx + dy * dy <= radius * radius
    }
}

#[cfg(test)]
mod tests {
    use crate::circle_and_rectangle_overlapping::Solution;

    #[test]
    fn test_check_overlap() {
        // radius = 1, xCenter = 0, yCenter = 0, x1 = 1, y1 = -1, x2 = 3, y2 = 1
        let ans = Solution::check_overlap(1, 0, 0, 1, -1, 3, 1);
        assert_eq!(ans, true);

        // radius = 1, xCenter = 1, yCenter = 1, x1 = 1, y1 = -3, x2 = 2, y2 = -1
        let ans = Solution::check_overlap(1, 1, 1, 1, -3, 2, -1);
        assert_eq!(ans, false);

        let ans = Solution::check_overlap(1, 1, 1, -3, -3, 3, 3);
        assert_eq!(ans, true);

        let ans = Solution::check_overlap(4, 102, 50, 0, 0, 100, 100);
        assert_eq!(ans, true);

        let ans = Solution::check_overlap(1415, 807, -784, -733, 623, -533, 1005);
        assert_eq!(ans, false);
    }
}
