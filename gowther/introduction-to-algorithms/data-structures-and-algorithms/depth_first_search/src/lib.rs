use std::{collections::HashMap, hash::Hash};

pub fn dfs<'a, Node>(
    adj: &HashMap<&'a Node, Vec<&'a Node>>,
    s: &'a Node,
    parent: Option<HashMap<&'a Node, &'a Node>>,
    order: Option<Vec<&'a Node>>,
) -> (HashMap<&'a Node, &'a Node>, Vec<&'a Node>)
where
    Node: PartialEq + Eq + Hash,
{
    let mut parent = parent.unwrap_or(HashMap::from([(s, s)]));
    let mut order = order.unwrap_or_default();
    for v in adj.get(s).unwrap() {
        if !parent.contains_key(v) {
            parent.insert(v, s);
            (parent, order) = dfs(adj, v, Some(parent), Some(order));
        }
    }
    order.push(s);
    return (parent, order);
}

pub fn dfs_all_vertexes<'a, Node>(
    adj: &HashMap<&'a Node, Vec<&'a Node>>,
) -> (HashMap<&'a Node, &'a Node>, Vec<&'a Node>)
where
    Node: PartialEq + Eq + Hash,
{
    let mut parent: HashMap<&'a Node, &'a Node> = HashMap::new();
    let mut order: Vec<&'a Node> = Vec::new();

    for v in adj.keys() {
        if !parent.contains_key(v) {
            parent.insert(v, v);
            (parent, order) = dfs(&adj, v, Some(parent), Some(order));
        }
    }

    return (parent, order);
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
    fn test_dfs_single_source() {
        let u = Node::new('u');
        let v = Node::new('v');
        let w = Node::new('w');
        let x = Node::new('x');
        let y = Node::new('y');
        let z = Node::new('z');
        let adj = HashMap::from([
            (&u, vec![&v, &x]),
            (&v, vec![&y]),
            (&w, vec![&y, &z]),
            (&x, vec![&v]),
            (&y, vec![&x]),
            (&z, vec![&z]),
        ]);
        let (parent, order) = dfs(&adj, &u, None, None);
        assert_eq!(parent.get(&u), Some(&&u));
        assert_eq!(parent.get(&v), Some(&&u));
        assert_eq!(parent.get(&w), None);
        assert_eq!(parent.get(&x), Some(&&y));
        assert_eq!(parent.get(&y), Some(&&v));
        assert_eq!(parent.get(&z), None);
        assert_eq!(order, [&x, &y, &v, &u]);
    }

    #[test]
    fn test_dfs_all_vertexes() {
        let u = Node::new('u');
        let v = Node::new('v');
        let w = Node::new('w');
        let x = Node::new('x');
        let y = Node::new('y');
        let z = Node::new('z');
        let adj = HashMap::from([
            (&u, vec![&v, &x]),
            (&v, vec![&y]),
            (&w, vec![&y, &z]),
            (&x, vec![&v]),
            (&y, vec![&x]),
            (&z, vec![&z]),
        ]);
        let (parent, order) = dfs_all_vertexes(&adj);
        assert_eq!(parent.len(), 6);
        assert_eq!(order.len(), 6);
    }
}
