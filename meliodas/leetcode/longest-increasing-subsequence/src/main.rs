// MIT 6.006 lecture 16: https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/28461a74f81101874a13d9679a40584d_MIT6_006S20_lec16.pdf
// leetcode 300: https://leetcode.com/problems/longest-increasing-subsequence/description/

struct Solution;

impl Solution {
    // Time: O(N^2)
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // Subproblems: x[i] is the length of the longest
        // increasing subsequence ending at i
        // Base: x[0] = 1
        let mut x: Vec<i32> = vec![1; n];

        // Topolocical order: range (N-1) to 0
        for i in 0..n {
            // Relate
            x[i] = 1
                + (0..i)
                    .filter(|&j| nums[i] > nums[j])
                    .map(|j| x[j])
                    .max()
                    .unwrap_or(0); // not greater than any left item
        }

        // Original problem
        *x.iter().max().unwrap()
    }
}

fn main() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
}
