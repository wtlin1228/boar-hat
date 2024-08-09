// MIT 6.006 Recitation 16: https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/487dc976eabd0f9bcd6a3c385203ea27_MIT6_006S20_r16.pdf
// leetcode 53: https://leetcode.com/problems/maximum-subarray/description/

struct Solution;

impl Solution {
    // Time: O(n)
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // Subproblem: x(k) is the max subarray sum ending at A[k]
        let mut x = Vec::with_capacity(n);

        // Base: x(0) = A[0]
        x.push(nums[0]);

        // Topological order: increase k
        for k in 1..n {
            // Relate: x(k) = max{ x(k-1) + A[k], A[k] }
            x.push(std::cmp::max(x[k - 1] + nums[k], nums[k]));
        }

        // Original problem
        *x.iter().max().unwrap()
    }
}

fn main() {
    assert_eq!(
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        6
    );

    assert_eq!(Solution::max_sub_array(vec![1]), 1);

    assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
}
