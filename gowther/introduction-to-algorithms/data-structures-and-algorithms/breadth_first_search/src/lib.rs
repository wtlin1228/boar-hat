use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

pub fn bfs<'a, Node>(
    adj: HashMap<&'a Node, Vec<&'a Node>>,
    s: &'a Node,
) -> HashMap<&'a Node, (&'a Node, usize)>
where
    Node: PartialEq + Eq + Hash,
{
    let mut parent_level_table: HashMap<&Node, (&Node, usize)> = HashMap::with_capacity(adj.len());
    let mut queue: VecDeque<&'a Node> = VecDeque::from([s]);
    parent_level_table.insert(s, (s, 0));
    while queue.len() > 0 {
        let u = queue.pop_front().unwrap();
        let u_level = parent_level_table.get(u).unwrap().1;
        for v in adj.get(u).unwrap() {
            if !parent_level_table.contains_key(v) {
                parent_level_table.insert(v, (u, u_level + 1));
                queue.push_back(v);
            }
        }
    }
    parent_level_table
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
    fn it_works() {
        let s = Node::new('s');
        let r = Node::new('r');
        let u = Node::new('u');
        let v = Node::new('v');
        let t = Node::new('t');
        let w = Node::new('w');
        let y = Node::new('y');
        let x = Node::new('x');
        let z = Node::new('z');
        let adj = HashMap::from([
            (&s, vec![&r, &u, &v]),
            (&r, vec![&s, &t, &w]),
            (&u, vec![&s, &t, &y]),
            (&v, vec![&s, &w, &y]),
            (&t, vec![&r, &u]),
            (&w, vec![&r, &v, &x, &z]),
            (&y, vec![&y, &v, &x]),
            (&x, vec![&w, &y, &z]),
            (&z, vec![&w, &x]),
        ]);
        let bfs_table = bfs(adj, &s);
        assert_eq!(bfs_table.len(), 9);
        assert_eq!(bfs_table.get(&s).unwrap(), &(&s, 0));
        assert_eq!(bfs_table.get(&r).unwrap(), &(&s, 1));
        assert_eq!(bfs_table.get(&u).unwrap(), &(&s, 1));
        assert_eq!(bfs_table.get(&v).unwrap(), &(&s, 1));
        assert_eq!(bfs_table.get(&t).unwrap(), &(&r, 2));
        assert_eq!(bfs_table.get(&w).unwrap(), &(&r, 2));
        assert_eq!(bfs_table.get(&y).unwrap(), &(&u, 2));
        assert_eq!(bfs_table.get(&x).unwrap(), &(&w, 3));
        assert_eq!(bfs_table.get(&z).unwrap(), &(&w, 3));
    }
}
