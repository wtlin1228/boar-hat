// MIT 6.006 Lecture 17: https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2020/665523227a175e9e9ce26ea8d3e5b51c_MIT6_006S20_lec17.pdf

enum Operator {
    Addition,
    Production,
}

impl Operator {
    fn eval(&self, left: i32, right: i32) -> i32 {
        match self {
            Operator::Addition => left + right,
            Operator::Production => left * right,
        }
    }
}

fn get_max(a: i32, b: i32, c: i32, d: i32) -> i32 {
    std::cmp::max(a, std::cmp::max(b, std::cmp::max(c, d)))
}

fn get_min(a: i32, b: i32, c: i32, d: i32) -> i32 {
    std::cmp::min(a, std::cmp::min(b, std::cmp::min(c, d)))
}

struct Solution;

impl Solution {
    // Idea: guess the last operator
    // Time: O(n^3)
    fn arithmetic_parenthesization(operands: Vec<i32>, operators: Vec<Operator>) -> (i32, i32) {
        let n = operands.len();

        // Subproblems:
        // - max(i, j) = maximum of a_i op_i a_i+1 ... op_j-1 a_j
        // - min(i, j) = minimum of a_i op_i a_i+1 ... op_j-1 a_j
        let mut max: Vec<Vec<i32>> = vec![vec![0; n]; n];
        let mut min: Vec<Vec<i32>> = vec![vec![0; n]; n];

        // Base: only one number, no operators left!
        for i in 0..n {
            max[i][i] = operands[i];
            min[i][i] = operands[i];
        }

        // Topological order: increase (j - i)
        for distance in 1..n {
            for i in 0..n {
                let j = i + distance;
                if j < n {
                    // Relate:
                    // - max(i, j) = max{
                    //     max(i, k) op_k max(k+1, j),
                    //     max(i, k) op_k min(k+1, j),
                    //     min(i, k) op_k max(k+1, j),
                    //     min(i, k) op_k min(k+1, j),
                    //   }, where i <= k <= j
                    // - min(i, j) = min{
                    //     max(i, k) op_k max(k+1, j),
                    //     max(i, k) op_k min(k+1, j),
                    //     min(i, k) op_k max(k+1, j),
                    //     min(i, k) op_k min(k+1, j),
                    //   }, where i <= k <= j
                    max[i][j] = operators[i].eval(max[i][i], max[i + 1][j]);
                    min[i][j] = operators[i].eval(max[i][i], max[i + 1][j]);
                    for k in i..j {
                        max[i][j] = std::cmp::max(
                            max[i][j],
                            get_max(
                                operators[k].eval(max[i][k], max[k + 1][j]),
                                operators[k].eval(max[i][k], min[k + 1][j]),
                                operators[k].eval(min[i][k], max[k + 1][j]),
                                operators[k].eval(min[i][k], min[k + 1][j]),
                            ),
                        );
                        min[i][j] = std::cmp::min(
                            min[i][j],
                            get_min(
                                operators[k].eval(max[i][k], max[k + 1][j]),
                                operators[k].eval(max[i][k], min[k + 1][j]),
                                operators[k].eval(min[i][k], max[k + 1][j]),
                                operators[k].eval(min[i][k], min[k + 1][j]),
                            ),
                        );
                    }
                }
            }
        }

        // Original problem
        (max[0][n - 1], min[0][n - 1])
    }
}

fn main() {
    assert_eq!(
        Solution::arithmetic_parenthesization(
            vec![7, -4, 3, -5],
            vec![Operator::Addition, Operator::Production, Operator::Addition]
        ),
        (15, -10)
    );

    assert_eq!(
        Solution::arithmetic_parenthesization(
            vec![1, 2, 3, 4, 5],
            vec![
                Operator::Addition,
                Operator::Production,
                Operator::Addition,
                Operator::Production,
            ]
        ),
        (105, 27)
    );
}
