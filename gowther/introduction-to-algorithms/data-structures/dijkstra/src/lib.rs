use std::{cmp, collections::HashMap, hash::Hash};

trait ChangeablePriorityQueue<'a, V> {
    fn insert(&mut self, vertex: &'a V, key: Option<i32>);
    fn extract_min(&mut self) -> &'a V;
    fn decrease_key(&mut self, vertex: &'a V, key: i32);
}

struct Item<'a, V> {
    vertex: &'a V,
    key: Option<i32>, // None for Infinity
}

impl<'a, V> PartialEq for Item<'a, V> {
    fn eq(&self, other: &Self) -> bool {
        match (self.key, other.key) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(a), Some(b)) => a == b,
        }
    }
}

impl<'a, V> PartialOrd for Item<'a, V> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self.key, other.key) {
            (None, None) => Some(cmp::Ordering::Equal), // Infinity = Infinity
            (None, Some(_)) => Some(cmp::Ordering::Greater), // Infinity > some integer
            (Some(_), None) => Some(cmp::Ordering::Less), // some integer < Infinity
            (Some(a), Some(b)) => Some(a.cmp(&b)),
        }
    }
}

impl<'a, V> Item<'a, V> {
    pub fn new(vertex: &'a V, key: Option<i32>) -> Self {
        Self { vertex, key }
    }
}

struct PriorityQueue<'a, V> {
    data: Vec<Item<'a, V>>,
    vertex2idx: HashMap<&'a V, usize>,
}

impl<'a, V> PriorityQueue<'a, V>
where
    V: PartialEq + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            vertex2idx: HashMap::new(),
        }
    }

    fn min_heapify_up(&mut self, c: usize) {
        if c == 0 {
            return;
        }
        let p: usize = (c - 1) / 2;
        if self.data[p] > self.data[c] {
            self.data.swap(c, p);
            self.vertex2idx.insert(self.data[c].vertex, c);
            self.vertex2idx.insert(self.data[p].vertex, p);
            self.min_heapify_up(p);
        }
    }

    fn min_heapify_down(&mut self, p: usize) {
        if p >= self.data.len() {
            return;
        }
        let mut l: usize = 2 * p + 1;
        let mut r: usize = 2 * p + 2;
        if l >= self.data.len() {
            l = p;
        }
        if r >= self.data.len() {
            r = p;
        }
        let c: usize = match self.data[r] > self.data[l] {
            true => l,
            false => r,
        };
        if self.data[p] > self.data[c] {
            self.data.swap(c, p);
            self.vertex2idx.insert(self.data[c].vertex, c);
            self.vertex2idx.insert(self.data[p].vertex, p);
            self.min_heapify_down(c);
        }
    }
}

impl<'a, V> ChangeablePriorityQueue<'a, V> for PriorityQueue<'a, V>
where
    V: PartialEq + Eq + Hash,
{
    fn insert(&mut self, vertex: &'a V, key: Option<i32>) {
        self.data.push(Item::new(vertex, key));
        let idx = self.data.len() - 1;
        self.vertex2idx.insert(self.data[idx].vertex, idx);
        self.min_heapify_up(idx);
    }

    fn extract_min(&mut self) -> &'a V {
        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);
        self.vertex2idx.insert(self.data[0].vertex, 0);
        self.vertex2idx.remove(self.data[last_idx].vertex);
        let min_vertex = self.data.pop().unwrap().vertex;
        self.min_heapify_down(0);

        min_vertex
    }

    fn decrease_key(&mut self, vertex: &'a V, key: i32) {
        if let Some(&idx) = self.vertex2idx.get(vertex) {
            if self.data[idx].key.is_none() || key < self.data[idx].key.unwrap() {
                self.data[idx].key = Some(key);
                self.min_heapify_up(idx);
            }
        }
    }
}

fn try_to_relax<'a, V>(
    w: &HashMap<(&'a V, &'a V), i32>,
    d: &mut HashMap<&'a V, Option<i32>>,
    parent: &mut HashMap<&'a V, Option<&'a V>>,
    u: &'a V,
    v: &'a V,
) where
    V: PartialEq + Eq + Hash,
{
    let d_s_u = d.get(u).unwrap().unwrap();
    let w_u_v = w.get(&(u, v)).unwrap().to_owned();
    if d.get(v).unwrap().is_none() || d.get(v).unwrap().unwrap() > d_s_u + w_u_v {
        d.insert(v, Some(d_s_u + w_u_v));
        parent.insert(v, Some(u));
    }
}

pub fn dijkstra<'a, V>(
    adj: &HashMap<&'a V, Vec<&'a V>>, // Assume it's a connected graph.
    w: &HashMap<(&'a V, &'a V), i32>,
    s: &'a V,
) -> (
    /* delta  */ HashMap<&'a V, Option<i32>>,
    /* parent */ HashMap<&'a V, Option<&'a V>>,
)
where
    V: PartialEq + Eq + Hash,
{
    let mut d: HashMap<&'a V, Option<i32>> = adj.keys().map(|&v| (v, None)).collect();
    let mut parent: HashMap<&'a V, Option<&'a V>> = adj.keys().map(|&v| (v, None)).collect();
    d.insert(s, Some(0));
    parent.insert(s, Some(s));

    let mut q = PriorityQueue::new();
    for &v in adj.keys() {
        q.insert(v, d.get(v).unwrap().to_owned());
    }

    for _ in 0..adj.len() {
        let u = q.extract_min();
        for &v in adj.get(u).unwrap().iter() {
            try_to_relax(w, &mut d, &mut parent, u, v);
            q.decrease_key(v, d.get(v).unwrap().unwrap());
        }
    }

    (d, parent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_changeable_priority_queue() {
        let mut q = PriorityQueue::new();
        q.insert(&"x", Some(9));
        q.insert(&"y", Some(8));
        q.insert(&"z", Some(7));
        q.insert(&"a", Some(1));
        q.insert(&"b", Some(2));
        q.insert(&"c", Some(3));

        assert_eq!(q.extract_min(), &"a");
        q.decrease_key(&"y", 1);
        assert_eq!(q.extract_min(), &"y");
        assert_eq!(q.extract_min(), &"b");
        assert_eq!(q.extract_min(), &"c");
        assert_eq!(q.extract_min(), &"z");
        assert_eq!(q.extract_min(), &"x");
    }

    #[test]
    fn test_changeable_priority_queue_with_infinity_key() {
        let mut q = PriorityQueue::new();
        q.insert(&"x", None);
        q.insert(&"y", None);
        q.insert(&"z", None);
        q.insert(&"a", None);
        q.insert(&"b", None);
        q.insert(&"c", None);

        q.decrease_key(&"x", 9);
        q.decrease_key(&"y", 8);
        q.decrease_key(&"z", 7);
        q.decrease_key(&"a", 1);
        q.decrease_key(&"b", 2);
        q.decrease_key(&"c", 3);

        assert_eq!(q.extract_min(), &"a");
        assert_eq!(q.extract_min(), &"b");
        assert_eq!(q.extract_min(), &"c");
        assert_eq!(q.extract_min(), &"z");
        assert_eq!(q.extract_min(), &"y");
        assert_eq!(q.extract_min(), &"x");
    }

    #[test]
    fn test_dijkstra() {
        let s = "s";
        let a = "a";
        let b = "b";
        let c = "c";
        let d = "d";
        let adj = HashMap::from([
            (&s, vec![&a, &c]),
            (&a, vec![&b, &c]),
            (&b, vec![&d]),
            (&c, vec![&a, &b, &d]),
            (&d, vec![&b]),
        ]);
        let weights = HashMap::from([
            ((&s, &a), 10),
            ((&s, &c), 3),
            ((&a, &b), 2),
            ((&a, &c), 1),
            ((&b, &d), 7),
            ((&c, &a), 4),
            ((&c, &b), 8),
            ((&c, &d), 2),
            ((&d, &b), 5),
        ]);

        let (delta, parent) = dijkstra(&adj, &weights, &s);

        assert_eq!(delta.get(&s).unwrap(), &Some(0));
        assert_eq!(delta.get(&a).unwrap(), &Some(7));
        assert_eq!(delta.get(&b).unwrap(), &Some(9));
        assert_eq!(delta.get(&c).unwrap(), &Some(3));
        assert_eq!(delta.get(&d).unwrap(), &Some(5));

        assert_eq!(parent.get(&s).unwrap(), &Some(&s));
        assert_eq!(parent.get(&a).unwrap(), &Some(&c));
        assert_eq!(parent.get(&b).unwrap(), &Some(&a));
        assert_eq!(parent.get(&c).unwrap(), &Some(&s));
        assert_eq!(parent.get(&d).unwrap(), &Some(&c));
    }
}
