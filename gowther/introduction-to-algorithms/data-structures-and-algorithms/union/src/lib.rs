use std::{
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct DisjointSetForests<T>
where
    T: Hash + Clone + Eq + PartialEq,
{
    nodes: HashMap<T, Rc<RefCell<Node>>>,
    pub set_count: usize,
    next_id: usize,
}

#[derive(Debug)]
struct Node {
    id: usize,
    rank: usize,
    parent: Option<Weak<RefCell<Node>>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Hash + Clone + Eq + PartialEq> DisjointSetForests<T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            set_count: 0,
            next_id: 0,
        }
    }

    pub fn make_set(&mut self, x: T) -> Result<(), String> {
        if self.nodes.contains_key(&x) {
            return Err(String::from("duplicated key"));
        }

        self.set_count += 1;
        self.next_id += 1;
        self.nodes.insert(
            x,
            Rc::new(RefCell::new(Node {
                id: self.next_id,
                rank: 0,
                parent: None,
            })),
        );

        Ok(())
    }

    pub fn union(&mut self, x: &T, y: &T) -> Result<(), String> {
        let node_x = self.nodes.get(x);
        let node_y = self.nodes.get(y);
        if node_x.is_none() || node_y.is_none() {
            return Err(String::from("key not found"));
        }

        let set_x = Self::find_set(Rc::clone(node_x.unwrap()));
        let set_y = Self::find_set(Rc::clone(node_y.unwrap()));

        if set_x == set_y {
            return Ok(());
        }

        self.set_count -= 1;
        Self::link(set_x, set_y);

        Ok(())
    }

    fn link(x: Rc<RefCell<Node>>, y: Rc<RefCell<Node>>) {
        let rank_x = x.borrow().rank;
        let rank_y = y.borrow().rank;
        match rank_x.cmp(&rank_y) {
            std::cmp::Ordering::Less => x.borrow_mut().parent = Some(Rc::downgrade(&y)),
            std::cmp::Ordering::Equal => {
                x.borrow_mut().parent = Some(Rc::downgrade(&y));
                y.borrow_mut().rank += 1;
            }
            std::cmp::Ordering::Greater => y.borrow_mut().parent = Some(Rc::downgrade(&x)),
        };
    }

    fn find_set(x: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let x_p = x.borrow_mut().parent.take();
        if x_p.is_none() {
            return x;
        }

        let set = Self::find_set(x_p.unwrap().upgrade().unwrap());
        x.borrow_mut().parent = Some(Rc::downgrade(&set));
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //
        //  a─────b     e
        //  │    /│     │
        //  │   / │     │
        //  │  /  │     │
        //  │ /   │     │
        //  c     d     f
        //              │
        //  h─────i     g
        //
        //        j
        //
        let mut union_set: DisjointSetForests<char> = DisjointSetForests::new();
        assert_eq!(union_set.nodes.len(), 0);
        assert_eq!(union_set.set_count, 0);

        // make sets
        for c in 'a'..='j' {
            union_set.make_set(c).unwrap();
        }
        assert_eq!(union_set.nodes.len(), 10);
        assert_eq!(union_set.set_count, 10);

        // apply edges
        union_set.union(&'b', &'d').unwrap();
        union_set.union(&'e', &'f').unwrap();
        union_set.union(&'a', &'c').unwrap();
        union_set.union(&'h', &'i').unwrap();
        union_set.union(&'a', &'b').unwrap();
        union_set.union(&'f', &'g').unwrap();
        union_set.union(&'b', &'c').unwrap();
        assert_eq!(union_set.nodes.len(), 10);
        assert_eq!(union_set.set_count, 4);

        // more assertions
        let set_a =
            DisjointSetForests::<char>::find_set(Rc::clone(union_set.nodes.get(&'a').unwrap()));
        let set_b =
            DisjointSetForests::<char>::find_set(Rc::clone(union_set.nodes.get(&'b').unwrap()));
        let set_c =
            DisjointSetForests::<char>::find_set(Rc::clone(union_set.nodes.get(&'c').unwrap()));
        let set_d =
            DisjointSetForests::<char>::find_set(Rc::clone(union_set.nodes.get(&'d').unwrap()));
        assert_eq!(set_a, set_b);
        assert_eq!(set_a, set_c);
        assert_eq!(set_a, set_d);

        let set_e =
            DisjointSetForests::<char>::find_set(Rc::clone(union_set.nodes.get(&'e').unwrap()));
        let set_f =
            DisjointSetForests::<char>::find_set(Rc::clone(union_set.nodes.get(&'f').unwrap()));
        let set_g =
            DisjointSetForests::<char>::find_set(Rc::clone(union_set.nodes.get(&'g').unwrap()));
        assert_eq!(set_e, set_f);
        assert_eq!(set_e, set_g);

        let set_h =
            DisjointSetForests::<char>::find_set(Rc::clone(union_set.nodes.get(&'h').unwrap()));
        let set_i =
            DisjointSetForests::<char>::find_set(Rc::clone(union_set.nodes.get(&'i').unwrap()));
        assert_eq!(set_h, set_i);
    }
}
