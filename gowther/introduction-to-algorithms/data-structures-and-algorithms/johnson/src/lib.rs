use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

use bellman_ford::{bellman_ford, Weight};
use dijkstra::dijkstra;

fn get_connected_graph<'a, V>(
    adj: &HashMap<&'a V, Vec<&'a V>>,
    s: &'a V,
) -> HashMap<&'a V, Vec<&'a V>>
where
    V: PartialEq + Eq + Hash,
{
    let mut visited: HashSet<&'a V> = HashSet::new();
    let mut queue: VecDeque<&'a V> = VecDeque::new();
    queue.push_back(s);
    visited.insert(s);
    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        for &v in adj.get(u).unwrap().iter() {
            if !visited.contains(v) {
                visited.insert(v);
                queue.push_back(v);
            }
        }
    }

    let mut connected_graph: HashMap<&'a V, Vec<&'a V>> = HashMap::new();
    for (&u, next) in adj.iter() {
        if visited.contains(u) {
            let mut tmp = Vec::new();
            for &v in next.iter() {
                if visited.contains(v) {
                    tmp.push(v);
                }
            }
            connected_graph.insert(u, tmp);
        }
    }

    connected_graph
}

/// Johnson’s Algorithm
/// - Construct G' from G by adding vertex x connected to each vertex v ∈ V with 0-weight edge
/// - Compute δ'(x, v) for every v ∈ V (using Bellman-Ford)
/// - If δ'(x, v) = −∞ for any v ∈ V :
///     - Abort (since there is a negative-weight cycle in G)
/// - Else:
///     - Re-weight each edge w'(u, v) = w(u, v) + δ'(x, u) − δ'(x, v) to form graph G'
///     - For each u ∈ V :
///         - Compute shortest-path distances δ'(u, v) to all v in G' (using Dijkstra)
///         - Compute δ(u, v) = δ'(u, v) − δ'(x, u) + δ'(x, v) for all v ∈ V
pub fn johnson<'a, V>(
    adj: &HashMap<&'a V, Vec<&'a V>>,
    w: &HashMap<(&'a V, &'a V), i32>,
    x: &'a V, // x is the super node, pass in here to meet Rust's lifetime check
) -> (
    /* delta  */ HashMap<&'a V, HashMap<&'a V, Option<i32>>>,
    /* parent */ HashMap<&'a V, HashMap<&'a V, Option<&'a V>>>,
)
where
    V: PartialEq + Eq + Hash + Debug,
{
    let mut adj_copied: HashMap<&V, Vec<&V>> = adj.to_owned();
    let mut w_copied: HashMap<(&V, &V), i32> = w.to_owned();
    adj_copied.insert(&x, adj.keys().map(|&v| v).collect::<Vec<&V>>());
    for &v in adj.keys() {
        w_copied.insert((&x, v), 0);
    }

    // println!("{:#?}", adj_copied);
    // println!("{:#?}", w_copied);

    let (d_x, _) = bellman_ford(&adj_copied, &w_copied, &x);
    if d_x.values().any(|w| w == &Weight::MinusInfinity) {
        panic!("Johnson's algorithm doesn't support graph with negative-weight cycle");
    }

    for (&u, next) in adj.iter() {
        let d_x_u = w_copied.get(&(&x, u)).unwrap().to_owned();
        for &v in next.iter() {
            let d_x_v = w_copied.get(&(&x, v)).unwrap().to_owned();
            let w_u_v = w.get(&(u, v)).unwrap().to_owned();
            *w_copied.get_mut(&(u, v)).unwrap() = w_u_v + d_x_u - d_x_v;
        }
    }

    let mut delta_table: HashMap<&V, HashMap<&V, Option<i32>>> =
        HashMap::with_capacity(adj.keys().len());
    let mut parent_table: HashMap<&V, HashMap<&V, Option<&V>>> =
        HashMap::with_capacity(adj.keys().len());

    for &u in adj.keys() {
        let d_x_u = w_copied.get(&(&x, u)).unwrap().to_owned();
        // let d_x_u = 2;
        let connected_graph = get_connected_graph(&adj, u);
        let (mut delta, parent) = dijkstra(&connected_graph, &w_copied, u);
        for (&v, d) in delta.iter_mut() {
            if let Some(d_u_v) = d {
                let d_x_v = w_copied.get(&(&x, v)).unwrap().to_owned();
                // let d_x_v = 1;
                *d_u_v = d_u_v.to_owned() - d_x_u + d_x_v;
            }
        }
        delta_table.insert(u, delta);
        parent_table.insert(u, parent);
    }

    (delta_table, parent_table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_johnson() {
        let a = "a";
        let b = "b";
        let c = "c";
        let d = "d";
        let e = "e";
        let adj = HashMap::from([
            (&a, vec![&b, &d]),
            (&b, vec![&c, &e]),
            (&c, vec![&a, &d]),
            (&d, vec![]),
            (&e, vec![&c, &d]),
        ]);
        let weights = HashMap::from([
            ((&a, &b), 2),
            ((&a, &d), 5),
            ((&b, &c), -2),
            ((&b, &e), 1),
            ((&c, &a), 0),
            ((&c, &d), 5),
            ((&e, &c), 2),
            ((&e, &d), 1),
        ]);
        let x = "x"; // super node

        let (delta, parent) = johnson(&adj, &weights, &x);

        assert_eq!(delta.get(&a).unwrap().get(&a).unwrap(), &Some(0));
        assert_eq!(delta.get(&a).unwrap().get(&b).unwrap(), &Some(2));
        assert_eq!(delta.get(&a).unwrap().get(&c).unwrap(), &Some(0));
        assert_eq!(delta.get(&a).unwrap().get(&d).unwrap(), &Some(4));
        assert_eq!(delta.get(&a).unwrap().get(&e).unwrap(), &Some(3));
        assert_eq!(parent.get(&a).unwrap().get(&a).unwrap(), &Some(&a));
        assert_eq!(parent.get(&a).unwrap().get(&b).unwrap(), &Some(&a));
        assert_eq!(parent.get(&a).unwrap().get(&c).unwrap(), &Some(&b));
        assert_eq!(parent.get(&a).unwrap().get(&d).unwrap(), &Some(&e));
        assert_eq!(parent.get(&a).unwrap().get(&e).unwrap(), &Some(&b));

        assert_eq!(delta.get(&b).unwrap().get(&a).unwrap(), &Some(-2));
        assert_eq!(delta.get(&b).unwrap().get(&b).unwrap(), &Some(0));
        assert_eq!(delta.get(&b).unwrap().get(&c).unwrap(), &Some(-2));
        assert_eq!(delta.get(&b).unwrap().get(&d).unwrap(), &Some(2));
        assert_eq!(delta.get(&b).unwrap().get(&e).unwrap(), &Some(1));
        assert_eq!(parent.get(&b).unwrap().get(&a).unwrap(), &Some(&c));
        assert_eq!(parent.get(&b).unwrap().get(&b).unwrap(), &Some(&b));
        assert_eq!(parent.get(&b).unwrap().get(&c).unwrap(), &Some(&b));
        assert_eq!(parent.get(&b).unwrap().get(&d).unwrap(), &Some(&e));
        assert_eq!(parent.get(&b).unwrap().get(&e).unwrap(), &Some(&b));

        assert_eq!(delta.get(&c).unwrap().get(&a).unwrap(), &Some(0));
        assert_eq!(delta.get(&c).unwrap().get(&b).unwrap(), &Some(2));
        assert_eq!(delta.get(&c).unwrap().get(&c).unwrap(), &Some(0));
        assert_eq!(delta.get(&c).unwrap().get(&d).unwrap(), &Some(4));
        assert_eq!(delta.get(&c).unwrap().get(&e).unwrap(), &Some(3));
        assert_eq!(parent.get(&c).unwrap().get(&a).unwrap(), &Some(&c));
        assert_eq!(parent.get(&c).unwrap().get(&b).unwrap(), &Some(&a));
        assert_eq!(parent.get(&c).unwrap().get(&c).unwrap(), &Some(&c));
        assert_eq!(parent.get(&c).unwrap().get(&d).unwrap(), &Some(&e));
        assert_eq!(parent.get(&c).unwrap().get(&e).unwrap(), &Some(&b));

        assert_eq!(delta.get(&d).unwrap().get(&a), None);
        assert_eq!(delta.get(&d).unwrap().get(&b), None);
        assert_eq!(delta.get(&d).unwrap().get(&c), None);
        assert_eq!(delta.get(&d).unwrap().get(&d).unwrap(), &Some(0));
        assert_eq!(delta.get(&d).unwrap().get(&e), None);
        assert_eq!(parent.get(&d).unwrap().get(&a), None);
        assert_eq!(parent.get(&d).unwrap().get(&b), None);
        assert_eq!(parent.get(&d).unwrap().get(&c), None);
        assert_eq!(parent.get(&d).unwrap().get(&d).unwrap(), &Some(&d));
        assert_eq!(parent.get(&d).unwrap().get(&e), None);

        assert_eq!(delta.get(&e).unwrap().get(&a).unwrap(), &Some(2));
        assert_eq!(delta.get(&e).unwrap().get(&b).unwrap(), &Some(4));
        assert_eq!(delta.get(&e).unwrap().get(&c).unwrap(), &Some(2));
        assert_eq!(delta.get(&e).unwrap().get(&d).unwrap(), &Some(1));
        assert_eq!(delta.get(&e).unwrap().get(&e).unwrap(), &Some(0));
        assert_eq!(parent.get(&e).unwrap().get(&a).unwrap(), &Some(&c));
        assert_eq!(parent.get(&e).unwrap().get(&b).unwrap(), &Some(&a));
        assert_eq!(parent.get(&e).unwrap().get(&c).unwrap(), &Some(&e));
        assert_eq!(parent.get(&e).unwrap().get(&d).unwrap(), &Some(&e));
        assert_eq!(parent.get(&e).unwrap().get(&e).unwrap(), &Some(&e));
    }

    #[test]
    #[should_panic(
        expected = "Johnson's algorithm doesn't support graph with negative-weight cycle"
    )]
    fn test_johnson_with_negative_circle() {
        let a = "a";
        let b = "b";
        let c = "c";
        let d = "d";
        let e = "e";
        let adj = HashMap::from([
            (&a, vec![&b, &d]),
            (&b, vec![&c, &e]),
            (&c, vec![&a, &d]),
            (&d, vec![]),
            (&e, vec![&c, &d]),
        ]);
        let weights = HashMap::from([
            ((&a, &b), 2),
            ((&a, &d), 5),
            ((&b, &c), -2),
            ((&b, &e), 1),
            ((&c, &a), -1),
            ((&c, &d), 5),
            ((&e, &c), 2),
            ((&e, &d), 1),
        ]);
        let x = "x"; // super node

        johnson(&adj, &weights, &x);
    }
}
