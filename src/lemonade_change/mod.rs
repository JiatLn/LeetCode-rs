/// <https://leetcode.cn/problems/lemonade-change/>
struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;
        for bill in bills {
            match bill {
                5 => {
                    five += 1;
                }
                10 => {
                    if five < 1 {
                        return false;
                    } else {
                        five -= 1;
                        ten += 1;
                    }
                }
                20 => {
                    if ten > 0 {
                        ten -= 1;
                        if five < 1 {
                            return false;
                        }
                        five -= 1;
                    } else {
                        if five < 3 {
                            return false;
                        }
                        five -= 3;
                    }
                }
                _ => todo!(),
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::lemonade_change::Solution;

    #[test]
    fn test_lemonade_change() {
        let ans = Solution::lemonade_change(vec![5, 5, 5, 10, 20]);
        assert_eq!(ans, true);
    }
}
