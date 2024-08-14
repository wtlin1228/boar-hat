// MIT 6.006 Lecture 17: https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/665523227a175e9e9ce26ea8d3e5b51c_MIT6_006S20_lec17.pdf

struct Solution;

impl Solution {
    // Idea: it's a DAG with (notes[i], f[j]) vertices
    // Time: O(n * F^2)
    fn piano_fingering<F>(notes: Vec<usize>, finger_number: usize, d: F) -> usize
    where
        // d(t, f, t', f') =  difficulty of transitioning from note t with finger f
        //                    to note t' with finger f'
        F: Fn(usize, usize, usize, usize) -> usize,
    {
        let n = notes.len();

        // Subproblems:
        // - x(i, f) = minimum total difficulty for playing notes t0, t1, ..., ti
        //   ending with finger f on note ti
        // - for 0 <= i < n and 0 <= f < F
        let mut x: Vec<Vec<usize>> = vec![vec![0; finger_number]; n];

        // Base: we can put any finger on the first note without difficulty
        for f in 0..finger_number {
            x[0][f] = 0;
        }

        // Topological order: increase t = 1, 2, ..., n - 1
        for i in 1..n {
            for f in 0..finger_number {
                // Relate:
                // - x(i, f) = min{ x(i-1, f_prev) + d(t[i-1], f_prev, t[i], f) | 0 <= f_prev < F }
                x[i][f] = x[i - 1]
                    .iter()
                    .enumerate()
                    .map(|(f_prev, value)| value + d(notes[i - 1], f_prev, notes[i], f))
                    .min()
                    .unwrap();
            }
        }

        // Original problem
        *x[n - 1].iter().min().unwrap()
    }
}

fn main() {
    assert_eq!(
        Solution::piano_fingering(
            vec![1, 2, 3],
            5,
            |t_prev: usize, f_prev: usize, t: usize, f: usize| -> usize {
                match (t_prev, f_prev, t, f) {
                    (_, _, _, _) => 9,
                }
            }
        ),
        18
    );
}
