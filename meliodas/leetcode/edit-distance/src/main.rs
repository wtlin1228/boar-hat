// MIT 6.006 Recitation 16: https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/487dc976eabd0f9bcd6a3c385203ea27_MIT6_006S20_r16.pdf
// leetcode 72: https://leetcode.com/problems/edit-distance/description/

struct Solution;

impl Solution {
    // Time: O(|A||B|), |A| = length of word1, |B| = length of word2
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let a = word1.len();
        let b = word2.len();

        // Subproblem: x(i, j) = edit distance to B[0..i] from A[0..j]
        let mut x: Vec<Vec<i32>> = vec![vec![0; a + 1]; b + 1];

        // Base
        for i in 0..=b {
            x[i][0] = i as i32;
        }
        for j in 0..=a {
            x[0][j] = j as i32;
        }

        // Topological order: inc j then i
        for i in 1..=b {
            for j in 1..=a {
                // Relate: x(i, j) = min{
                //   do replace: x(i-1, j-1) + 1, (don't need to plus 1 if A[j] == B[i])
                //   do delete:  x(i, j-1) + 1,
                //   do insert:  x(i-1, j) + 1
                // }
                x[i][j] = std::cmp::min(
                    x[i - 1][j - 1]
                        + match &word1[j - 1..j] == &word2[i - 1..i] {
                            true => 0,
                            false => 1,
                        },
                    std::cmp::min(x[i][j - 1] + 1, x[i - 1][j] + 1),
                )
            }
        }

        // Original problem is x(b, a) = edit distance to B from A
        x[b][a]
    }
}

fn main() {
    assert_eq!(
        Solution::min_distance("horse".to_string(), "ros".to_string()),
        3
    );

    assert_eq!(
        Solution::min_distance("intention".to_string(), "execution".to_string()),
        5
    );
}
