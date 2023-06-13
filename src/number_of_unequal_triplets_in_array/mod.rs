/// <https://leetcode.cn/problems/number-of-unequal-triplets-in-array/>
struct Solution;

impl Solution {
    pub fn unequal_triplets(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        nums.sort();
        let len = nums.len();
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                if nums[i] == nums[j] {
                    continue;
                }
                for k in j + 1..len {
                    // nums[i] < nums[j] <= nums[k] 无需比较 i,k
                    if nums[j] != nums[k] {
                        ans += len - k;
                        break;
                    }
                }
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::number_of_unequal_triplets_in_array::Solution;
    #[test]
    fn test_unequal_triplets() {
        let ans = Solution::unequal_triplets(vec![4, 4, 2, 4, 3]);
        assert_eq!(ans, 3);

        let ans = Solution::unequal_triplets(vec![1, 1, 1, 1, 1]);
        assert_eq!(ans, 0);
    }
}
