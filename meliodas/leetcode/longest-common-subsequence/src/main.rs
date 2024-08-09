// MIT 6.006 lecture 16: https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/28461a74f81101874a13d9679a40584d_MIT6_006S20_lec16.pdf
// leetcode 1143: https://leetcode.com/problems/longest-common-subsequence/description/

struct Solution;

impl Solution {
    // Time: O(|A| * |B|)
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let a = text1.len();
        let b = text2.len();

        // Subproblems: x(i, j) = LCS(A[i:], B[j:])
        // Base: x[i, a] = 0 = x[b, j]
        let mut x: Vec<Vec<i32>> = vec![vec![0; b + 1]; a + 1];

        // Topological order: decrease j then i
        for i in (0..a).rev() {
            for j in (0..b).rev() {
                // Relate
                x[i][j] = match &text1[i..i + 1] == &text2[j..j + 1] {
                    true => 1 + x[i + 1][j + 1],
                    false => std::cmp::max(x[i + 1][j], x[i][j + 1]),
                }
            }
        }

        // Original problem
        x[0][0]
    }
}

fn main() {
    assert_eq!(
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
        3
    );

    assert_eq!(
        Solution::longest_common_subsequence("abcde".to_string(), "".to_string()),
        0
    );

    assert_eq!(
        Solution::longest_common_subsequence("abcde".to_string(), "xywoo".to_string()),
        0
    );
}
