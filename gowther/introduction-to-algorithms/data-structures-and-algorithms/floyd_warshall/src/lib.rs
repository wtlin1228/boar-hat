use std::{collections::HashMap, hash::Hash};

// Time: O(|V|^3)
pub fn floyd_warshall<'a, V>(
    // Expect the graph has no negative cycle, this can be checked by applying
    // Bellman-Ford for the graph.
    adj: &HashMap<&'a V, Vec<&'a V>>,
    w: &HashMap<(&'a V, &'a V), i32>,
) -> HashMap<&'a V, HashMap<&'a V, Option<i32>>>
where
    V: PartialEq + Eq + Hash,
{
    let v_len: usize = adj.len();

    // Subproblems:
    //   d(u, v, k) = minimum weight of a path from u to v that only uses
    //   vertices from {1, 2, . . . , k} ∪ {u, v}. For u, v ∈ V and 1 ≤ k ≤ |V|.
    let mut d: Vec<Vec<Vec<Option<i32>>>> = vec![vec![vec![None; v_len + 1]; v_len + 1]; v_len + 1];

    // Base:
    //   d(u, u, 0) = 0
    //   d(u, v, 0) = w(u, v) if (u, v) ∈ E
    //   d(u, v, 0) = ∞ if none of above
    for (u, &vertex_u) in adj.keys().enumerate() {
        for (v, &vertex_v) in adj.keys().enumerate() {
            d[0][u + 1][v + 1] = match vertex_u == vertex_v {
                true => Some(0),
                false => match w.get(&(vertex_u, vertex_v)) {
                    Some(weight) => Some(*weight),
                    None => None,
                },
            }
        }
    }

    // Topological order: increase k from 1 to |V|
    for k in 1..=v_len {
        for u in 1..=v_len {
            for v in 1..=v_len {
                // Relate:
                //   d(u, v, k) = min{
                //     d(u, v, k-1), ⬅️ without vertex k
                //     d(u, k, k-1) + d(k, v, k-1) ⬅️ with vertex k
                //   }
                d[k][u][v] = match (d[k - 1][u][v], d[k - 1][u][k], d[k - 1][k][v]) {
                    (None, None, None) => None,
                    (None, None, Some(_)) => None,
                    (None, Some(_), None) => None,
                    (None, Some(u2k), Some(k2v)) => Some(u2k + k2v),
                    (Some(u2v), None, None) => Some(u2v),
                    (Some(u2v), None, Some(_)) => Some(u2v),
                    (Some(u2v), Some(_), None) => Some(u2v),
                    (Some(u2v), Some(u2k), Some(k2v)) => Some(std::cmp::min(u2v, u2k + k2v)),
                }
            }
        }
    }

    // Original problem: d(u, v, |V|) for all u, v ∈ V
    let mut delta: HashMap<&'a V, HashMap<&'a V, Option<i32>>> = HashMap::with_capacity(v_len);
    for (u, &vertex_u) in adj.keys().enumerate() {
        let mut u2v_table: HashMap<&'a V, Option<i32>> = HashMap::with_capacity(v_len);
        for (v, &vertex_v) in adj.keys().enumerate() {
            u2v_table.insert(vertex_v, d[v_len][u + 1][v + 1]);
        }
        delta.insert(vertex_u, u2v_table);
    }

    delta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "a";
        let b = "b";
        let c = "c";
        let d = "d";
        let e = "e";
        let adj = HashMap::from([
            (&a, vec![&b, &c, &e]),
            (&b, vec![&d, &e]),
            (&c, vec![&d]),
            (&d, vec![&a]),
            (&e, vec![&b, &c]),
        ]);
        let weights = HashMap::from([
            ((&a, &b), 3),
            ((&a, &c), 2),
            ((&a, &e), 2),
            ((&b, &d), 1),
            ((&b, &e), 2),
            ((&c, &d), 1),
            ((&d, &a), 4),
            ((&e, &b), 1),
            ((&e, &c), 1),
        ]);

        let delta = floyd_warshall(&adj, &weights);

        assert_eq!(delta.get(&a).unwrap().get(&a).unwrap(), &Some(0));
        assert_eq!(delta.get(&a).unwrap().get(&b).unwrap(), &Some(3));
        assert_eq!(delta.get(&a).unwrap().get(&c).unwrap(), &Some(2));
        assert_eq!(delta.get(&a).unwrap().get(&d).unwrap(), &Some(3));
        assert_eq!(delta.get(&a).unwrap().get(&e).unwrap(), &Some(2));

        assert_eq!(delta.get(&b).unwrap().get(&a).unwrap(), &Some(5));
        assert_eq!(delta.get(&b).unwrap().get(&e).unwrap(), &Some(2));
        assert_eq!(delta.get(&b).unwrap().get(&c).unwrap(), &Some(3));
        assert_eq!(delta.get(&b).unwrap().get(&b).unwrap(), &Some(0));
        assert_eq!(delta.get(&b).unwrap().get(&d).unwrap(), &Some(1));

        assert_eq!(delta.get(&c).unwrap().get(&d).unwrap(), &Some(1));
        assert_eq!(delta.get(&c).unwrap().get(&b).unwrap(), &Some(8));
        assert_eq!(delta.get(&c).unwrap().get(&a).unwrap(), &Some(5));
        assert_eq!(delta.get(&c).unwrap().get(&e).unwrap(), &Some(7));
        assert_eq!(delta.get(&c).unwrap().get(&c).unwrap(), &Some(0));

        assert_eq!(delta.get(&d).unwrap().get(&a).unwrap(), &Some(4));
        assert_eq!(delta.get(&d).unwrap().get(&c).unwrap(), &Some(6));
        assert_eq!(delta.get(&d).unwrap().get(&d).unwrap(), &Some(0));
        assert_eq!(delta.get(&d).unwrap().get(&b).unwrap(), &Some(7));
        assert_eq!(delta.get(&d).unwrap().get(&e).unwrap(), &Some(6));

        assert_eq!(delta.get(&e).unwrap().get(&d).unwrap(), &Some(2));
        assert_eq!(delta.get(&e).unwrap().get(&c).unwrap(), &Some(1));
        assert_eq!(delta.get(&e).unwrap().get(&e).unwrap(), &Some(0));
        assert_eq!(delta.get(&e).unwrap().get(&a).unwrap(), &Some(6));
        assert_eq!(delta.get(&e).unwrap().get(&b).unwrap(), &Some(1));
    }
}
