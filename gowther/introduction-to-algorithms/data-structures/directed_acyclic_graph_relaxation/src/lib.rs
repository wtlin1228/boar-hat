use depth_first_search::dfs_all_vertexes;
use std::{collections::HashMap, hash::Hash};

fn topological_sort<'a, Node>(adj: &HashMap<&'a Node, Vec<&'a Node>>) -> Vec<&'a Node>
where
    Node: PartialEq + Eq + Hash,
{
    let (_, mut order) = dfs_all_vertexes(adj);
    order.reverse();
    order
}

pub fn dag_relaxation<'a, Node>(
    adj: &HashMap<&'a Node, Vec<&'a Node>>,
    weights: &HashMap<(&'a Node, &'a Node), i32>,
    s: &'a Node,
) -> HashMap<&'a Node, &'a Node>
where
    Node: PartialEq + Eq + Hash,
{
    let topological_order: Vec<&Node> = topological_sort(adj);
    let mut parent = HashMap::from([(s, s)]);
    let mut distance_estimate: HashMap<(&Node, &Node), i32> = HashMap::from([((s, s), 0)]);
    for &u in topological_order.iter() {
        let outgoing_neighbors = adj.get(u).unwrap();
        for &v in outgoing_neighbors.iter() {
            let d_s_u = distance_estimate.get(&(s, u)).unwrap();
            let w_u_v = *weights.get(&(u, v)).unwrap();
            let d_s_v = distance_estimate.get(&(s, v));
            if d_s_v.is_none() || *d_s_v.unwrap() > d_s_u + w_u_v {
                distance_estimate.insert((s, v), d_s_u + w_u_v);
                parent.insert(v, u);
            }
        }
    }
    parent
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
    fn test_dfs_all_vertexes() {
        let a = Node::new('a');
        let b = Node::new('b');
        let c = Node::new('c');
        let d = Node::new('d');
        let e = Node::new('e');
        let f = Node::new('f');
        let g = Node::new('g');
        let h = Node::new('h');
        let adj = HashMap::from([
            (&a, vec![&b, &e]),
            (&b, vec![&c, &e, &f]),
            (&c, vec![]),
            (&d, vec![&c]),
            (&e, vec![&f]),
            (&f, vec![&c, &g]),
            (&g, vec![&c, &h]),
            (&h, vec![&c, &d]),
        ]);
        let weights = HashMap::from([
            ((&a, &b), -5),
            ((&a, &e), 7),
            ((&b, &c), -1),
            ((&b, &e), 6),
            ((&b, &f), -4),
            ((&d, &c), 5),
            ((&e, &f), 3),
            ((&f, &c), 8),
            ((&f, &g), 2),
            ((&g, &c), 1),
            ((&g, &h), -2),
            ((&h, &c), 9),
            ((&h, &d), 4),
        ]);

        let parent = dag_relaxation(&adj, &weights, &a);

        assert_eq!(*parent.get(&f).unwrap(), &b);
        assert_eq!(*parent.get(&c).unwrap(), &b);
        assert_eq!(*parent.get(&g).unwrap(), &f);
        assert_eq!(*parent.get(&h).unwrap(), &g);
        assert_eq!(*parent.get(&e).unwrap(), &b);
        assert_eq!(*parent.get(&d).unwrap(), &h);
        assert_eq!(*parent.get(&a).unwrap(), &a);
        assert_eq!(*parent.get(&b).unwrap(), &a);
    }
}
