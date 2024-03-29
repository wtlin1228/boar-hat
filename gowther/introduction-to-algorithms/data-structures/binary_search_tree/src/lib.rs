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
    fn unlink_parent(&self);
    fn unlink_left(&self);
    fn unlink_right(&self);
    fn set_left(&self, left: &Rc<RefCell<Node<T, K>>>);
    fn set_right(&self, right: &Rc<RefCell<Node<T, K>>>);
    fn is_left_child(&self) -> bool;
    fn is_right_child(&self) -> bool;
    fn is_leaf(&self) -> bool;
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

    fn unlink_parent(&self) {
        if self.is_left_child() {
            self.parent().unwrap().borrow_mut().left.take();
        } else if self.is_right_child() {
            self.parent().unwrap().borrow_mut().right.take();
        }
        self.borrow_mut().parent.take();
    }

    fn unlink_left(&self) {
        if self.borrow().left.is_some() {
            let left = self.borrow_mut().left.take().unwrap();
            left.borrow_mut().parent.take();
        }
    }

    fn unlink_right(&self) {
        if self.borrow().right.is_some() {
            let right = self.borrow_mut().right.take().unwrap();
            right.borrow_mut().parent.take();
        }
    }

    fn set_left(&self, left: &Rc<RefCell<Node<T, K>>>) {
        let left = Rc::clone(left);

        self.unlink_left();
        self.borrow_mut().left = Some(Rc::clone(&left));

        left.unlink_parent();
        left.borrow_mut().parent = Some(Rc::downgrade(self));
    }

    fn set_right(&self, right: &Rc<RefCell<Node<T, K>>>) {
        let right = Rc::clone(right);

        self.unlink_right();
        self.borrow_mut().right = Some(Rc::clone(&right));

        right.unlink_parent();
        right.borrow_mut().parent = Some(Rc::downgrade(self));
    }

    fn is_left_child(&self) -> bool {
        match self.parent() {
            Some(parent) => match parent.left() {
                Some(left) => self == &left,
                None => false,
            },
            None => false,
        }
    }

    fn is_right_child(&self) -> bool {
        match self.parent() {
            Some(parent) => match parent.right() {
                Some(right) => self == &right,
                None => false,
            },
            None => false,
        }
    }

    fn is_leaf(&self) -> bool {
        self.left().is_none() && self.right().is_none()
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
                    if x.is_left_child() {
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
                    if x.is_right_child() {
                        return Some(Rc::clone(node));
                    }
                }
                None => return None,
            }
            x = Rc::clone(y.as_ref().unwrap());
            y = x.parent();
        }
    }

    fn transplant(&mut self, u: &Rc<RefCell<Node<T, K>>>, v: Option<Rc<RefCell<Node<T, K>>>>) {
        if let Some(ref v) = v {
            v.unlink_parent();
        }

        if u.parent().is_none() {
            self.root = match v {
                Some(ref node) => Some(Rc::clone(node)),
                None => None,
            };
            return;
        } else if u.is_left_child() {
            match v {
                Some(ref node) => u.parent().unwrap().set_left(node),
                None => u.parent().unwrap().borrow_mut().left = None,
            }
        } else {
            match v {
                Some(ref node) => u.parent().unwrap().set_right(node),
                None => u.parent().unwrap().borrow_mut().right = None,
            }
        }

        u.unlink_parent();
    }

    pub fn insert(&mut self, node: Node<T, K>) {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(node)));
            return;
        }

        let mut x = Some(Rc::clone(self.root.as_ref().unwrap()));
        let mut y: Option<Rc<RefCell<Node<T, K>>>> = None;
        let mut should_insert_to_left_of_y = false;
        while x.is_some() {
            y = x;
            match node.key() < y.as_ref().unwrap().borrow().key() {
                true => {
                    x = y.as_ref().unwrap().left();
                    should_insert_to_left_of_y = true;
                }
                false => {
                    x = y.as_ref().unwrap().right();
                    should_insert_to_left_of_y = false;
                }
            }
        }

        match should_insert_to_left_of_y {
            true => y.as_ref().unwrap().set_left(&Rc::new(RefCell::new(node))),
            false => y.as_ref().unwrap().set_right(&Rc::new(RefCell::new(node))),
        };
    }

    pub fn delete(&mut self, k: &K) -> Result<Rc<RefCell<Node<T, K>>>, ()> {
        // it's an empty tree
        if self.root.is_none() {
            return Err(());
        }

        // find the node to delete
        let root = self.root.as_ref().unwrap();
        let z = BinarySearchTree::search(Some(Rc::clone(root)), k);
        if z.is_none() {
            return Err(());
        }

        // z is the node to delete
        let z = z.unwrap();

        if z.left().is_none() {
            self.transplant(&z, z.right());
        } else if z.right().is_none() {
            self.transplant(&z, z.left());
        } else {
            let y = BinarySearchTree::minimum(&z.right().unwrap());
            // y.left is guaranteed to be Nil since it's the minimum of z.right
            if y != z.right().unwrap() {
                self.transplant(&y, y.right());
                y.set_right(z.right().as_ref().unwrap());
            }
            self.transplant(&z, Some(Rc::clone(&y)));
            y.set_left(z.left().as_ref().unwrap());
        }

        Ok(z)
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

    #[test]
    fn test_transplant_root() {
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

        let mut bst = BinarySearchTree {
            root: Some(Rc::clone(&n_15)),
        };
        bst.transplant(&n_15, Some(Rc::clone(&n_18)));

        assert_eq!(n_15.parent(), None);
        assert_eq!(n_15.left(), Some(Rc::clone(&n_6)));
        assert_eq!(n_15.right(), None);

        assert_eq!(bst.root, Some(Rc::clone(&n_18)));
        assert_eq!(bst.root.as_ref().unwrap().left(), Some(Rc::clone(&n_17)));
        assert_eq!(bst.root.as_ref().unwrap().right(), Some(Rc::clone(&n_20)));
    }

    #[test]
    fn test_transplant_not_root() {
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

        let mut bst = BinarySearchTree {
            root: Some(Rc::clone(&n_15)),
        };
        bst.transplant(&n_6, Some(Rc::clone(&n_18)));

        assert_eq!(n_6.parent(), None);
        assert_eq!(n_6.left(), Some(Rc::clone(&n_3)));
        assert_eq!(n_6.right(), Some(Rc::clone(&n_7)));

        assert_eq!(bst.root, Some(Rc::clone(&n_15)));
        assert_eq!(bst.root.as_ref().unwrap().left(), Some(Rc::clone(&n_18)));
        assert_eq!(bst.root.as_ref().unwrap().right(), None);

        assert_eq!(n_18.parent(), Some(Rc::clone(&n_15)));
        assert_eq!(n_18.left(), Some(Rc::clone(&n_17)));
        assert_eq!(n_18.right(), Some(Rc::clone(&n_20)));
    }

    #[test]
    fn test_insert() {
        let mut bst: BinarySearchTree<i32, i32> = BinarySearchTree { root: None };
        bst.insert(Node::new(15));
        bst.insert(Node::new(6));
        bst.insert(Node::new(18));
        bst.insert(Node::new(17));
        bst.insert(Node::new(20));
        bst.insert(Node::new(3));
        bst.insert(Node::new(7));
        bst.insert(Node::new(13));
        bst.insert(Node::new(9));
        bst.insert(Node::new(2));
        bst.insert(Node::new(4));

        assert_eq!(bst.root.as_ref().unwrap().borrow().data, 15);
        let n_6 = bst.root.as_ref().unwrap().left().unwrap();
        assert_eq!(n_6.borrow().data, 6);
        assert_eq!(n_6.parent(), bst.root);
        let n_3 = n_6.left().unwrap();
        assert_eq!(n_3.borrow().data, 3);
        assert_eq!(n_3.parent(), Some(Rc::clone(&n_6)));
        let n_2 = n_3.left().unwrap();
        assert_eq!(n_2.borrow().data, 2);
        assert_eq!(n_2.parent(), Some(Rc::clone(&n_3)));
        assert!(n_2.is_leaf());
        let n_4 = n_3.right().unwrap();
        assert_eq!(n_4.borrow().data, 4);
        assert_eq!(n_4.parent(), Some(Rc::clone(&n_3)));
        assert!(n_4.is_leaf());
        let n_7 = n_6.right().unwrap();
        assert_eq!(n_7.borrow().data, 7);
        assert_eq!(n_7.parent(), Some(Rc::clone(&n_6)));
        assert_eq!(n_7.left(), None);
        let n_13 = n_7.right().unwrap();
        assert_eq!(n_13.borrow().data, 13);
        assert_eq!(n_13.parent(), Some(Rc::clone(&n_7)));
        let n_9 = n_13.left().unwrap();
        assert_eq!(n_9.borrow().data, 9);
        assert_eq!(n_9.parent(), Some(Rc::clone(&n_13)));
        assert!(n_9.is_leaf());
        assert_eq!(n_13.right(), None);
        let n_18 = bst.root.as_ref().unwrap().right().unwrap();
        assert_eq!(n_18.borrow().data, 18);
        assert_eq!(n_18.parent(), bst.root);
        let n_17 = n_18.left().unwrap();
        assert_eq!(n_17.borrow().data, 17);
        assert_eq!(n_17.parent(), Some(Rc::clone(&n_18)));
        assert!(n_17.is_leaf());
        let n_20 = n_18.right().unwrap();
        assert_eq!(n_20.borrow().data, 20);
        assert_eq!(n_20.parent(), Some(Rc::clone(&n_18)));
        assert!(n_20.is_leaf());
    }

    #[test]
    fn test_delete() {
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

        let mut bst = BinarySearchTree {
            root: Some(Rc::clone(&n_15)),
        };

        let n_15 = bst.delete(&15).unwrap();
        assert_eq!(n_15.parent(), None);
        assert_eq!(n_15.left(), None);
        assert_eq!(n_15.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_17)));
        assert_eq!(n_17.parent(), None);
        assert_eq!(n_17.left(), Some(Rc::clone(&n_6)));
        assert_eq!(n_17.right(), Some(Rc::clone(&n_18)));
        assert_eq!(n_6.parent(), Some(Rc::clone(&n_17)));
        assert_eq!(n_18.parent(), Some(Rc::clone(&n_17)));

        let n_17 = bst.delete(&17).unwrap();
        assert_eq!(n_17.parent(), None);
        assert_eq!(n_17.left(), None);
        assert_eq!(n_17.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_18)));
        assert_eq!(n_18.parent(), None);
        assert_eq!(n_18.left(), Some(Rc::clone(&n_6)));
        assert_eq!(n_18.right(), Some(Rc::clone(&n_20)));
        assert_eq!(n_6.parent(), Some(Rc::clone(&n_18)));
        assert_eq!(n_20.parent(), Some(Rc::clone(&n_18)));

        let n_18 = bst.delete(&18).unwrap();
        assert_eq!(n_18.parent(), None);
        assert_eq!(n_18.left(), None);
        assert_eq!(n_18.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_20)));
        assert_eq!(n_20.parent(), None);
        assert_eq!(n_20.left(), Some(Rc::clone(&n_6)));
        assert_eq!(n_20.right(), None);
        assert_eq!(n_6.parent(), Some(Rc::clone(&n_20)));

        let n_20 = bst.delete(&20).unwrap();
        assert_eq!(n_20.parent(), None);
        assert_eq!(n_20.left(), None);
        assert_eq!(n_20.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_6)));
        assert_eq!(n_6.parent(), None);
        assert_eq!(n_6.left(), Some(Rc::clone(&n_3)));
        assert_eq!(n_6.right(), Some(Rc::clone(&n_7)));
        assert_eq!(n_3.parent(), Some(Rc::clone(&n_6)));
        assert_eq!(n_7.parent(), Some(Rc::clone(&n_6)));

        let n_2 = bst.delete(&2).unwrap();
        assert_eq!(n_2.parent(), None);
        assert_eq!(n_2.left(), None);
        assert_eq!(n_2.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_6)));
        assert_eq!(n_3.parent(), Some(Rc::clone(&n_6)));
        assert_eq!(n_3.left(), None);
        assert_eq!(n_3.right(), Some(Rc::clone(&n_4)));
        assert_eq!(n_6.left(), Some(Rc::clone(&n_3)));

        let n_3 = bst.delete(&3).unwrap();
        assert_eq!(n_3.parent(), None);
        assert_eq!(n_3.left(), None);
        assert_eq!(n_3.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_6)));
        assert_eq!(n_4.parent(), Some(Rc::clone(&n_6)));
        assert_eq!(n_4.left(), None);
        assert_eq!(n_4.right(), None);
        assert_eq!(n_6.left(), Some(Rc::clone(&n_4)));

        let n_4 = bst.delete(&4).unwrap();
        assert_eq!(n_4.parent(), None);
        assert_eq!(n_4.left(), None);
        assert_eq!(n_4.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_6)));
        assert_eq!(n_6.parent(), None);
        assert_eq!(n_6.left(), None);
        assert_eq!(n_6.right(), Some(Rc::clone(&n_7)));

        let n_13 = bst.delete(&13).unwrap();
        assert_eq!(n_13.parent(), None);
        assert_eq!(n_13.left(), None);
        assert_eq!(n_13.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_6)));
        assert_eq!(n_7.parent(), Some(Rc::clone(&n_6)));
        assert_eq!(n_7.left(), None);
        assert_eq!(n_7.right(), Some(Rc::clone(&n_9)));
        assert_eq!(n_9.parent(), Some(Rc::clone(&n_7)));
        assert_eq!(n_9.left(), None);
        assert_eq!(n_9.right(), None);

        let n_6 = bst.delete(&6).unwrap();
        assert_eq!(n_6.parent(), None);
        assert_eq!(n_6.left(), None);
        assert_eq!(n_6.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_7)));
        assert_eq!(n_7.parent(), None);
        assert_eq!(n_7.left(), None);
        assert_eq!(n_7.right(), Some(Rc::clone(&n_9)));
        assert_eq!(n_9.parent(), Some(Rc::clone(&n_7)));
        assert_eq!(n_9.left(), None);
        assert_eq!(n_9.right(), None);

        let n_9 = bst.delete(&9).unwrap();
        assert_eq!(n_9.parent(), None);
        assert_eq!(n_9.left(), None);
        assert_eq!(n_9.right(), None);
        assert_eq!(bst.root, Some(Rc::clone(&n_7)));
        assert_eq!(n_7.parent(), None);
        assert_eq!(n_7.left(), None);
        assert_eq!(n_7.right(), None);

        let n_7 = bst.delete(&7).unwrap();
        assert_eq!(n_7.parent(), None);
        assert_eq!(n_7.left(), None);
        assert_eq!(n_7.right(), None);
        assert_eq!(bst.root, None);

        assert_eq!(bst.delete(&1), Err(()));
    }
}
