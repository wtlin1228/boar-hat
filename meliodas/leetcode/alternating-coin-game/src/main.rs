// MIT 6.006 Lecture 16: https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/28461a74f81101874a13d9679a40584d_MIT6_006S20_lec16.pdf
// leetcode 486: https://leetcode.com/problems/predict-the-winner/description/

struct Solution;

impl Solution {
    // Time: O(n^2), n = length of nums
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();

        // Subproblems:
        // - me(left, right) = the best score I can get when I start from [v_left, ..., v_right]
        // - you(left, right) = the best score I can get when you start from [v_left, ..., v_right]
        let mut me: Vec<Vec<i32>> = vec![vec![0; n]; n];
        let mut you: Vec<Vec<i32>> = vec![vec![0; n]; n];

        // Base
        for i in 0..n {
            me[i][i] = nums[i];
            you[i][i] = 0;
        }

        // Topological order:
        // - distance = 1, ..., n-1
        // - left = 0, ..., n-1
        // - me then you
        for distance in 1..n {
            for left in 0..n {
                // Relate:
                // - me(left, right) = max{A[left] + you(left+1, right), A[right] + you(left, right-1)}
                // - you(left, right) = max{me(left+1, right), me(left, right-1)}
                let right = left + distance;
                if right < n {
                    me[left][right] = std::cmp::max(
                        nums[left] + you[left + 1][right],
                        nums[right] + you[left][right - 1],
                    );
                    you[left][right] = std::cmp::min(me[left + 1][right], me[left][right - 1]);
                }
            }
        }

        // Original problem: me(0, n-1) is the maximum score I can get
        me[0][n - 1] * 2 >= nums.iter().sum::<i32>()
    }
}

fn main() {
    assert_eq!(Solution::predict_the_winner(vec![1, 3, 1]), false);

    assert_eq!(Solution::predict_the_winner(vec![1, 5, 2]), false);

    assert_eq!(Solution::predict_the_winner(vec![1, 5, 233, 7]), true);
}
