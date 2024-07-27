use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Weight {
    Int(i32),
    MinusInfinity,
}

/// This version of Bellman-Ford based on graph duplication and DAG Relaxation that
/// solves SSSPs in O(|V||E|) time and space, and can return a negative-weight cycle
/// reachable on a path from s to v, for any vertex v with δ(s, v) = −∞.
///
/// - Construct new DAG G'=(V',E') from G=(V,E)
///     - G' has |V|(|V|+1) vertices v_k for all v ∈ V and k ∈ {0,...,|V|}
///     - G' has |V|(|V|+|E|) edges:
///         - |V| edges (v_k−1, v_k) for k ∈ {1,...,|V|} of weight zero for each v ∈ V
///         - |E| edges (u_k−1, v_k) for k ∈ {1,...,|V|} of weight w(u, v) for each (u, v) ∈ E
/// - Run DAG Relaxation on G' from s_0 to compute δ(s_0, v_k) for all v_k ∈ V'
/// - For each vertex: set d(s, v) = δ(s0, v_|V|−1)
/// - For each witness u ∈ V where δ(s0, u_|V|) < δ(s_0, u_|V|−1):
///     - For each vertex v reachable from u in G
///         - set d(s, v) = -∞
///
pub fn bellman_ford<'a, V>(
    adj: &HashMap<&'a V, Vec<&'a V>>, // adj should contain only the reachable vertices from s
    w: &HashMap<(&'a V, &'a V), i32>,
    s: &'a V,
) -> (
    /* delta  */ HashMap<&'a V, Weight>,
    /* parent */ HashMap<&'a V, &'a V>,
)
where
    V: PartialEq + Eq + Hash,
{
    let v_len = adj.len(); // |V|
    let mut witness: HashSet<&'a V> = HashSet::new();
    let mut delta: HashMap<&'a V, Weight> = HashMap::with_capacity(v_len);
    let mut parent: HashMap<&'a V, &'a V> = HashMap::with_capacity(v_len);
    let mut curr: HashMap<&'a V, Option<i32>> = HashMap::with_capacity(v_len);

    parent.insert(s, s);

    for &v in adj.keys() {
        curr.insert(
            v,
            match v == s {
                true => Some(0),
                false => None,
            },
        );
    }

    for k in 1..=v_len {
        // Edges (v_k-1, v_k) have zero weight.
        let mut next = curr.clone();
        for &u in adj.keys() {
            let d_u = curr.get(u).unwrap();
            if d_u.is_none() {
                // It is not reachable from s_0.
                continue;
            }
            let d_u = d_u.unwrap();
            for &v in adj.get(u).unwrap().iter() {
                let w_u_v = w.get(&(u, v)).unwrap().to_owned();
                if next.get(v).unwrap().is_none() || (next.get(v).unwrap().unwrap() > d_u + w_u_v) {
                    *next.get_mut(v).unwrap() = Some(d_u + w_u_v);
                    parent.insert(v, u);
                }
            }
        }
        if k == v_len {
            for (&v, d_s_v) in curr.iter() {
                // All vertices must be reachable from s_0 after |V| - 1 steps.
                delta.insert(v, Weight::Int(d_s_v.unwrap()));
            }
            for (&u, d_s_u) in next.iter() {
                // All vertices must be reachable from s_0 after |V| steps.
                let d_s_u = d_s_u.unwrap();
                let d_s_u_prev = curr.get(u).unwrap().unwrap();
                if d_s_u < d_s_u_prev {
                    // u is witness.
                    // Find all reachable vertices v from u in G and set d(s, v) = −∞
                    if witness.contains(u) {
                        continue;
                    }
                    witness.insert(u);
                    let mut queue: VecDeque<&V> = VecDeque::from([u]);
                    while queue.len() > 0 {
                        let u = queue.pop_front().unwrap();
                        delta.insert(u, Weight::MinusInfinity);
                        for &v in adj.get(u).unwrap().iter() {
                            if !witness.contains(v) {
                                witness.insert(v);
                                queue.push_back(v);
                            }
                        }
                    }
                }
            }
        }
        curr = next;
    }

    (delta, parent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq, Hash)]
    struct Node {
        id: char,
    }

    impl Node {
        pub fn new(c: char) -> Self {
            Self { id: c }
        }
    }

    #[test]
    fn test_no_negative_cycle() {
        let a = Node::new('a');
        let b = Node::new('b');
        let c = Node::new('c');
        let d = Node::new('d');
        let adj = HashMap::from([
            (&a, vec![&b, &c]),
            (&b, vec![&c]),
            (&c, vec![&d]),
            (&d, vec![]),
        ]);
        let weights = HashMap::from([((&a, &b), -5), ((&a, &c), 6), ((&b, &c), -4), ((&c, &d), 3)]);

        let (delta, parent) = bellman_ford(&adj, &weights, &a);
        assert_eq!(delta.get(&a), Some(Weight::Int(0)).as_ref());
        assert_eq!(delta.get(&b), Some(Weight::Int(-5)).as_ref());
        assert_eq!(delta.get(&c), Some(Weight::Int(-9)).as_ref());
        assert_eq!(delta.get(&d), Some(Weight::Int(-6)).as_ref());
        assert_eq!(parent.get(&a), Some(&a).as_ref());
        assert_eq!(parent.get(&b), Some(&a).as_ref());
        assert_eq!(parent.get(&c), Some(&b).as_ref());
        assert_eq!(parent.get(&d), Some(&c).as_ref());
    }

    #[test]
    fn test_negative_cycle() {
        let a = Node::new('a');
        let b = Node::new('b');
        let c = Node::new('c');
        let d = Node::new('d');
        let adj = HashMap::from([
            (&a, vec![&b, &c]),
            (&b, vec![&c]),
            (&c, vec![&d]),
            (&d, vec![&b]),
        ]);
        let weights = HashMap::from([
            ((&a, &b), -5),
            ((&a, &c), 6),
            ((&b, &c), -4),
            ((&c, &d), 3),
            ((&d, &b), -1),
        ]);

        let (delta, parent) = bellman_ford(&adj, &weights, &a);
        assert_eq!(delta.get(&a), Some(Weight::Int(0)).as_ref());
        assert_eq!(delta.get(&b), Some(Weight::MinusInfinity).as_ref());
        assert_eq!(delta.get(&c), Some(Weight::MinusInfinity).as_ref());
        assert_eq!(delta.get(&d), Some(Weight::MinusInfinity).as_ref());
        assert_eq!(parent.get(&a), Some(&a).as_ref());
        assert_eq!(parent.get(&b), Some(&d).as_ref());
        assert_eq!(parent.get(&c), Some(&b).as_ref());
        assert_eq!(parent.get(&d), Some(&c).as_ref());
    }
}
