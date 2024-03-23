use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub trait Key<T: Ord> {
    fn key(&self) -> &T;
}

#[derive(Debug)]
pub struct Node<T: Key<K> + PartialEq, K: Ord> {
    data: T,
    parent: Option<Weak<RefCell<Node<T, K>>>>,
    left: Option<Rc<RefCell<Node<T, K>>>>,
    right: Option<Rc<RefCell<Node<T, K>>>>,
}

impl<T: Key<K> + PartialEq, K: Ord> PartialEq for Node<T, K> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && {
                match (&self.parent, &other.parent) {
                    (Some(_), Some(_)) => self
                        .parent
                        .as_ref()
                        .unwrap()
                        .ptr_eq(other.parent.as_ref().unwrap()),
                    (None, None) => true,
                    (_, _) => false,
                }
            }
            && self.left == other.left
            && self.right == other.right
    }
}

impl<T: Key<K> + PartialEq, K: Ord> Key<K> for Node<T, K> {
    fn key(&self) -> &K {
        self.data.key()
    }
}

impl<T: Key<K> + PartialEq, K: Ord> Node<T, K> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            parent: None,
            left: None,
            right: None,
        }
    }
}

pub trait TreePointerHelper<T: Key<K> + PartialEq, K: Ord> {
    fn parent(&self) -> Option<Rc<RefCell<Node<T, K>>>>;
    fn left(&self) -> Option<Rc<RefCell<Node<T, K>>>>;
    fn right(&self) -> Option<Rc<RefCell<Node<T, K>>>>;
    fn set_left(&self, left: &Rc<RefCell<Node<T, K>>>);
    fn set_right(&self, right: &Rc<RefCell<Node<T, K>>>);
}

impl<T: Key<K> + PartialEq, K: Ord> TreePointerHelper<T, K> for Rc<RefCell<Node<T, K>>> {
    fn parent(&self) -> Option<Rc<RefCell<Node<T, K>>>> {
        match self.borrow().parent {
            Some(_) => Some(self.borrow().parent.as_ref().unwrap().upgrade().unwrap()),
            None => None,
        }
    }

    fn left(&self) -> Option<Rc<RefCell<Node<T, K>>>> {
        match self.borrow().left {
            Some(_) => Some(Rc::clone(self.borrow().left.as_ref().unwrap())),
            None => None,
        }
    }

    fn right(&self) -> Option<Rc<RefCell<Node<T, K>>>> {
        match self.borrow().right {
            Some(_) => Some(Rc::clone(self.borrow().right.as_ref().unwrap())),
            None => None,
        }
    }

    fn set_left(&self, left: &Rc<RefCell<Node<T, K>>>) {
        let left = Rc::clone(left);

        match self.left() {
            Some(prev_left) => {
                prev_left.borrow_mut().parent = None;
                self.borrow_mut().left = Some(Rc::clone(&left));
            }
            None => {
                self.borrow_mut().left = Some(Rc::clone(&left));
            }
        }

        match left.parent() {
            Some(prev_parent) => {
                if prev_parent.left().is_some() && prev_parent.left().unwrap() == left {
                    prev_parent.borrow_mut().left = None;
                } else {
                    prev_parent.borrow_mut().right = None;
                }
                left.borrow_mut().parent = Some(Rc::downgrade(self));
            }
            None => {
                left.borrow_mut().parent = Some(Rc::downgrade(self));
            }
        }
    }

    fn set_right(&self, right: &Rc<RefCell<Node<T, K>>>) {
        let right = Rc::clone(right);

        match self.right() {
            Some(prev_right) => {
                prev_right.borrow_mut().parent = None;
                self.borrow_mut().right = Some(Rc::clone(&right));
            }
            None => {
                self.borrow_mut().right = Some(Rc::clone(&right));
            }
        }

        match right.parent() {
            Some(prev_parent) => {
                if prev_parent.right().is_some() && prev_parent.right().unwrap() == right {
                    prev_parent.borrow_mut().right = None;
                } else {
                    prev_parent.borrow_mut().left = None;
                }
                right.borrow_mut().parent = Some(Rc::downgrade(self));
            }
            None => {
                right.borrow_mut().parent = Some(Rc::downgrade(self));
            }
        }
    }
}

#[derive(Debug)]
pub struct BinarySearchTree<T: Key<K> + PartialEq, K: Ord> {
    root: Option<Rc<RefCell<Node<T, K>>>>,
}

impl<T: Key<K> + PartialEq, K: Ord> BinarySearchTree<T, K> {
    pub fn search(x: Option<Rc<RefCell<Node<T, K>>>>, k: &K) -> Option<Rc<RefCell<Node<T, K>>>> {
        let mut x = x;
        loop {
            let next: Option<Rc<RefCell<Node<T, K>>>>;
            match x {
                Some(ref node) => match k.cmp(node.borrow().key()) {
                    std::cmp::Ordering::Less => next = node.left(),
                    std::cmp::Ordering::Equal => return Some(Rc::clone(node)),
                    std::cmp::Ordering::Greater => next = node.right(),
                },
                None => return None,
            }
            x = next;
        }
    }

    pub fn minimum(x: &Rc<RefCell<Node<T, K>>>) -> Rc<RefCell<Node<T, K>>> {
        let mut x = Rc::clone(x);
        loop {
            match x.left() {
                Some(left) => x = Rc::clone(&left),
                None => break,
            }
        }
        x
    }

    pub fn maximum(x: &Rc<RefCell<Node<T, K>>>) -> Rc<RefCell<Node<T, K>>> {
        let mut x = Rc::clone(x);
        loop {
            match x.right() {
                Some(right) => x = Rc::clone(&right),
                None => break,
            }
        }
        x
    }

    pub fn successor(x: &Rc<RefCell<Node<T, K>>>) -> Option<Rc<RefCell<Node<T, K>>>> {
        if let Some(right) = x.right() {
            return Some(Self::minimum(&right));
        }
        let mut x = Rc::clone(x);
        let mut y = x.parent();
        loop {
            match y {
                Some(ref node) => {
                    if node.left().is_some() && node.left().unwrap() == x {
                        return Some(Rc::clone(node));
                    }
                }
                None => return None,
            }
            x = Rc::clone(y.as_ref().unwrap());
            y = x.parent();
        }
    }

    pub fn predecessor(x: &Rc<RefCell<Node<T, K>>>) -> Option<Rc<RefCell<Node<T, K>>>> {
        if let Some(left) = x.left() {
            return Some(Self::maximum(&left));
        }
        let mut x = Rc::clone(x);
        let mut y = x.parent();
        loop {
            match y {
                Some(ref node) => {
                    if node.right().is_some() && node.right().unwrap() == x {
                        return Some(Rc::clone(node));
                    }
                }
                None => return None,
            }
            x = Rc::clone(y.as_ref().unwrap());
            y = x.parent();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Key<i32> for i32 {
        fn key(&self) -> &i32 {
            self
        }
    }

    #[derive(Debug, PartialEq)]
    struct Animal {
        id: u32,
        name: String,
    }

    impl Key<u32> for Animal {
        fn key(&self) -> &u32 {
            &self.id
        }
    }

    #[test]
    fn test_node_comparison() {
        let hawk_mama = Rc::new(RefCell::new(Node::new(Animal {
            id: 5,
            name: "Hawk Mama".to_string(),
        })));
        let hawk = Rc::new(RefCell::new(Node::new(Animal {
            id: 7,
            name: "Hawk".to_string(),
        })));
        let ben = Rc::new(RefCell::new(Node::new(Animal {
            id: 10,
            name: "Ben".to_string(),
        })));

        // hawk_mama
        //          \
        //           hawk
        //               \
        //                Ben
        hawk_mama.set_right(&hawk);
        hawk.set_right(&ben);

        assert_eq!(hawk_mama.right().unwrap(), hawk);
        assert_eq!(hawk_mama.right().unwrap().right().unwrap(), ben);
        assert_eq!(
            hawk_mama.right().unwrap().right().unwrap(),
            hawk.right().unwrap()
        );

        assert_eq!(hawk.parent().unwrap(), hawk_mama);
        assert_eq!(ben.parent().unwrap().parent().unwrap(), hawk_mama);
        assert_eq!(
            ben.parent().unwrap().parent().unwrap(),
            hawk.parent().unwrap()
        );
    }

    #[test]
    fn test_search() {
        let n_2 = Rc::new(RefCell::new(Node::new(2)));
        let n_3 = Rc::new(RefCell::new(Node::new(3)));
        let n_4 = Rc::new(RefCell::new(Node::new(4)));
        let n_6 = Rc::new(RefCell::new(Node::new(6)));
        let n_7 = Rc::new(RefCell::new(Node::new(7)));
        let n_13 = Rc::new(RefCell::new(Node::new(13)));
        let n_9 = Rc::new(RefCell::new(Node::new(9)));
        let n_15 = Rc::new(RefCell::new(Node::new(15)));
        let n_17 = Rc::new(RefCell::new(Node::new(17)));
        let n_18 = Rc::new(RefCell::new(Node::new(18)));
        let n_20 = Rc::new(RefCell::new(Node::new(20)));

        n_3.set_left(&n_2);
        n_3.set_right(&n_4);
        n_13.set_left(&n_9);
        n_7.set_right(&n_13);
        n_6.set_left(&n_3);
        n_6.set_right(&n_7);
        n_18.set_left(&n_17);
        n_18.set_right(&n_20);
        n_15.set_left(&n_6);
        n_15.set_right(&n_18);

        assert_eq!(
            BinarySearchTree::search(Some(Rc::clone(&n_15)), &9)
                .as_ref()
                .unwrap()
                .borrow()
                .data,
            n_9.borrow().data
        );
    }

    #[test]
    fn test_minimum() {
        let n_2 = Rc::new(RefCell::new(Node::new(2)));
        let n_3 = Rc::new(RefCell::new(Node::new(3)));
        let n_4 = Rc::new(RefCell::new(Node::new(4)));
        let n_6 = Rc::new(RefCell::new(Node::new(6)));
        let n_7 = Rc::new(RefCell::new(Node::new(7)));
        let n_13 = Rc::new(RefCell::new(Node::new(13)));
        let n_9 = Rc::new(RefCell::new(Node::new(9)));
        let n_15 = Rc::new(RefCell::new(Node::new(15)));
        let n_17 = Rc::new(RefCell::new(Node::new(17)));
        let n_18 = Rc::new(RefCell::new(Node::new(18)));
        let n_20 = Rc::new(RefCell::new(Node::new(20)));

        n_3.set_left(&n_2);
        n_3.set_right(&n_4);
        n_13.set_left(&n_9);
        n_7.set_right(&n_13);
        n_6.set_left(&n_3);
        n_6.set_right(&n_7);
        n_18.set_left(&n_17);
        n_18.set_right(&n_20);
        n_15.set_left(&n_6);
        n_15.set_right(&n_18);

        assert_eq!(
            BinarySearchTree::minimum(&n_15).borrow().data,
            n_2.borrow().data
        );
    }

    #[test]
    fn test_maximum() {
        let n_2 = Rc::new(RefCell::new(Node::new(2)));
        let n_3 = Rc::new(RefCell::new(Node::new(3)));
        let n_4 = Rc::new(RefCell::new(Node::new(4)));
        let n_6 = Rc::new(RefCell::new(Node::new(6)));
        let n_7 = Rc::new(RefCell::new(Node::new(7)));
        let n_13 = Rc::new(RefCell::new(Node::new(13)));
        let n_9 = Rc::new(RefCell::new(Node::new(9)));
        let n_15 = Rc::new(RefCell::new(Node::new(15)));
        let n_17 = Rc::new(RefCell::new(Node::new(17)));
        let n_18 = Rc::new(RefCell::new(Node::new(18)));
        let n_20 = Rc::new(RefCell::new(Node::new(20)));

        n_3.set_left(&n_2);
        n_3.set_right(&n_4);
        n_13.set_left(&n_9);
        n_7.set_right(&n_13);
        n_6.set_left(&n_3);
        n_6.set_right(&n_7);
        n_18.set_left(&n_17);
        n_18.set_right(&n_20);
        n_15.set_left(&n_6);
        n_15.set_right(&n_18);

        assert_eq!(
            BinarySearchTree::maximum(&n_15).borrow().data,
            n_20.borrow().data
        );
    }

    #[test]
    fn test_successor() {
        let n_2 = Rc::new(RefCell::new(Node::new(2)));
        let n_3 = Rc::new(RefCell::new(Node::new(3)));
        let n_4 = Rc::new(RefCell::new(Node::new(4)));
        let n_6 = Rc::new(RefCell::new(Node::new(6)));
        let n_7 = Rc::new(RefCell::new(Node::new(7)));
        let n_13 = Rc::new(RefCell::new(Node::new(13)));
        let n_9 = Rc::new(RefCell::new(Node::new(9)));
        let n_15 = Rc::new(RefCell::new(Node::new(15)));
        let n_17 = Rc::new(RefCell::new(Node::new(17)));
        let n_18 = Rc::new(RefCell::new(Node::new(18)));
        let n_20 = Rc::new(RefCell::new(Node::new(20)));

        n_3.set_left(&n_2);
        n_3.set_right(&n_4);
        n_13.set_left(&n_9);
        n_7.set_right(&n_13);
        n_6.set_left(&n_3);
        n_6.set_right(&n_7);
        n_18.set_left(&n_17);
        n_18.set_right(&n_20);
        n_15.set_left(&n_6);
        n_15.set_right(&n_18);

        assert_eq!(
            BinarySearchTree::successor(&n_15).unwrap().borrow().data,
            n_17.borrow().data
        );
        assert_eq!(
            BinarySearchTree::successor(&n_13).unwrap().borrow().data,
            n_15.borrow().data
        );
        assert_eq!(
            BinarySearchTree::successor(&n_4).unwrap().borrow().data,
            n_6.borrow().data
        );
        assert_eq!(BinarySearchTree::successor(&n_20), None);
    }

    #[test]
    fn test_predecessor() {
        let n_2 = Rc::new(RefCell::new(Node::new(2)));
        let n_3 = Rc::new(RefCell::new(Node::new(3)));
        let n_4 = Rc::new(RefCell::new(Node::new(4)));
        let n_6 = Rc::new(RefCell::new(Node::new(6)));
        let n_7 = Rc::new(RefCell::new(Node::new(7)));
        let n_13 = Rc::new(RefCell::new(Node::new(13)));
        let n_9 = Rc::new(RefCell::new(Node::new(9)));
        let n_15 = Rc::new(RefCell::new(Node::new(15)));
        let n_17 = Rc::new(RefCell::new(Node::new(17)));
        let n_18 = Rc::new(RefCell::new(Node::new(18)));
        let n_20 = Rc::new(RefCell::new(Node::new(20)));

        n_3.set_left(&n_2);
        n_3.set_right(&n_4);
        n_13.set_left(&n_9);
        n_7.set_right(&n_13);
        n_6.set_left(&n_3);
        n_6.set_right(&n_7);
        n_18.set_left(&n_17);
        n_18.set_right(&n_20);
        n_15.set_left(&n_6);
        n_15.set_right(&n_18);

        assert_eq!(
            BinarySearchTree::predecessor(&n_15).unwrap().borrow().data,
            n_13.borrow().data
        );
        assert_eq!(
            BinarySearchTree::predecessor(&n_13).unwrap().borrow().data,
            n_9.borrow().data
        );
        assert_eq!(
            BinarySearchTree::predecessor(&n_4).unwrap().borrow().data,
            n_3.borrow().data
        );
        assert_eq!(BinarySearchTree::predecessor(&n_2), None);
    }
}
