use crate::Key;

use super::node::{AVLTreeNode, Pointer};

use std::{
    cell::RefCell,
    mem::swap,
    rc::{Rc, Weak},
    slice::Iter,
};

trait Set<T, K>
where
    T: Key<K>,
    K: Ord,
{
    // container
    fn build(iter: Iter<T>) -> Self;
    fn len(&self) -> usize;

    // static
    fn find(&self, k: &K) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>>;

    // dynamic
    fn insert(&mut self, x: T);
    fn delete(&mut self, k: &K) -> Rc<RefCell<SetAVLTreeNode<T, K>>>;

    // order
    // iter_ord() -> Iter<T>;
    fn find_min(&self) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>>;
    fn find_max(&self) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>>;
    fn find_next(&self, k: &K) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>>;
    fn find_prev(&self, k: &K) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>>;
}

#[derive(Debug)]
struct SetAVLTreeNode<T, K> {
    data: T,

    // pointers
    parent: Option<Weak<RefCell<SetAVLTreeNode<T, K>>>>,
    left: Option<Rc<RefCell<SetAVLTreeNode<T, K>>>>,
    right: Option<Rc<RefCell<SetAVLTreeNode<T, K>>>>,

    // subtree properties
    height: i32,
}

impl<T, K> SetAVLTreeNode<T, K> {
    fn new_pointer(x: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            data: x,
            parent: None,
            left: None,
            right: None,
            height: 0,
        }))
    }
}

impl<T, K> PartialEq for SetAVLTreeNode<T, K>
where
    T: PartialEq,
{
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
            && self.height == other.height
    }
}

impl<T, K> Pointer for Rc<RefCell<SetAVLTreeNode<T, K>>>
where
    T: PartialEq,
{
    fn clone_pointer(&self) -> Self {
        Rc::clone(&self)
    }

    fn get_parent(&self) -> Option<Self> {
        match self.borrow().parent {
            Some(ref parent) => Some(parent.upgrade().unwrap()),
            None => None,
        }
    }

    fn get_left(&self) -> Option<Self> {
        match self.borrow().left {
            Some(ref left) => Some(left.clone_pointer()),
            None => None,
        }
    }

    fn get_right(&self) -> Option<Self> {
        match self.borrow().right {
            Some(ref right) => Some(right.clone_pointer()),
            None => None,
        }
    }

    fn set_left(&self, left: &Self) -> Option<Self> {
        let original_left = self.unlink_left();
        self.borrow_mut().left = Some(left.clone_pointer());
        left.unlink_parent();
        left.borrow_mut().parent = Some(Rc::downgrade(self));
        original_left
    }

    fn set_right(&self, right: &Self) -> Option<Self> {
        let original_right = self.unlink_right();
        self.borrow_mut().right = Some(right.clone_pointer());
        right.unlink_parent();
        right.borrow_mut().parent = Some(Rc::downgrade(self));
        original_right
    }

    fn take_parent(&self) -> Option<Self> {
        self.borrow_mut().parent.take().unwrap().upgrade()
    }

    fn take_left(&self) -> Option<Self> {
        self.borrow_mut().left.take()
    }

    fn take_right(&self) -> Option<Self> {
        self.borrow_mut().right.take()
    }
}

impl<T, K> AVLTreeNode for Rc<RefCell<SetAVLTreeNode<T, K>>>
where
    T: PartialEq + std::fmt::Debug,
    K: std::fmt::Debug,
{
    fn get_height(x: Option<Self>) -> i32 {
        match x {
            Some(x) => x.borrow().height,
            None => -1,
        }
    }

    fn set_height(&self, height: i32) {
        self.borrow_mut().height = height;
    }

    fn swap_item(x: &Self, y: &Self) {
        swap(&mut x.borrow_mut().data, &mut y.borrow_mut().data);
    }
}

#[derive(Debug)]
struct SetAVLTree<T, K> {
    root: Option<Rc<RefCell<SetAVLTreeNode<T, K>>>>,
    len: usize,
}

impl<T, K> SetAVLTree<T, K> {
    fn new() -> Self {
        Self { root: None, len: 0 }
    }
}

impl<T, K> Set<T, K> for SetAVLTree<T, K>
where
    T: PartialEq + Key<K> + std::fmt::Debug,
    K: Ord + std::fmt::Debug,
{
    fn build(iter: Iter<T>) -> Self {
        todo!()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn find(&self, k: &K) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>> {
        if self.root.is_none() {
            return None;
        }
        let mut x = Some(self.root.as_ref().unwrap().clone_pointer());
        loop {
            match x {
                Some(node) => match k.cmp(node.borrow().data.key()) {
                    std::cmp::Ordering::Less => x = node.get_left(),
                    std::cmp::Ordering::Equal => return Some(node.clone_pointer()),
                    std::cmp::Ordering::Greater => x = node.get_right(),
                },
                None => return None,
            }
        }
    }

    fn insert(&mut self, x: T) {
        self.len += 1;
        let x = SetAVLTreeNode::new_pointer(x);
        match self.root {
            None => self.root = Some(x),
            Some(ref root) => {
                let mut y = root.clone_pointer();
                let should_insert_before_y;
                loop {
                    let next;
                    match x.borrow().data.key().cmp(y.borrow().data.key()) {
                        std::cmp::Ordering::Less => match y.get_left() {
                            Some(left) => next = left,
                            None => {
                                // y.subtree_insert_before(x.clone_pointer());
                                should_insert_before_y = true;
                                break;
                            }
                        },
                        std::cmp::Ordering::Equal => {
                            unimplemented!("not support same key insertion")
                        }
                        std::cmp::Ordering::Greater => match y.get_right() {
                            Some(right) => next = right,
                            None => {
                                should_insert_before_y = false;
                                break;
                            }
                        },
                    };
                    y = next;
                }
                match should_insert_before_y {
                    true => {
                        y.subtree_insert_before(x.clone_pointer());
                    }
                    false => {
                        y.subtree_insert_after(x.clone_pointer());
                    }
                }
            }
        }
    }

    fn delete(&mut self, k: &K) -> Rc<RefCell<SetAVLTreeNode<T, K>>> {
        self.len -= 1;
        let delete_target = self.find(k).expect("key is not found");
        delete_target.subtree_delete();
        if self.len == 0 {
            self.root.take();
        }
        delete_target
    }

    fn find_min(&self) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>> {
        Some(
            self.root
                .as_ref()
                .expect("it's impossible get a minimum from an empty tree")
                .subtree_first(),
        )
    }

    fn find_max(&self) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>> {
        Some(
            self.root
                .as_ref()
                .expect("it's impossible get a maximum from an empty tree")
                .subtree_last(),
        )
    }

    fn find_next(&self, k: &K) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>> {
        let mut y = self
            .root
            .as_ref()
            .expect("it's impossible to find next from an empty tree")
            .clone_pointer();
        loop {
            let next;
            match k.cmp(y.borrow().data.key()) {
                std::cmp::Ordering::Less => match y.get_left() {
                    Some(left) => next = left,
                    None => return Some(y.clone_pointer()),
                },
                std::cmp::Ordering::Equal => return y.successor(),
                std::cmp::Ordering::Greater => match y.get_right() {
                    Some(right) => next = right,
                    None => return y.successor(),
                },
            };
            y = next;
        }
    }

    fn find_prev(&self, k: &K) -> Option<Rc<RefCell<SetAVLTreeNode<T, K>>>> {
        let mut y = self
            .root
            .as_ref()
            .expect("it's impossible to find prev from an empty tree")
            .clone_pointer();
        loop {
            let next;
            match k.cmp(y.borrow().data.key()) {
                std::cmp::Ordering::Less => match y.get_left() {
                    Some(left) => next = left,
                    None => return y.predecessor(),
                },
                std::cmp::Ordering::Equal => return y.successor(),
                std::cmp::Ordering::Greater => match y.get_right() {
                    Some(right) => next = right,
                    None => return Some(y.clone_pointer()),
                },
            };
            y = next;
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

    trait TestUtils {
        fn get_data(&self) -> i32;
        fn get_left(&self) -> Option<Rc<RefCell<SetAVLTreeNode<i32, i32>>>>;
        fn get_right(&self) -> Option<Rc<RefCell<SetAVLTreeNode<i32, i32>>>>;
    }

    impl TestUtils for Option<Rc<RefCell<SetAVLTreeNode<i32, i32>>>> {
        fn get_data(&self) -> i32 {
            self.as_ref().unwrap().borrow().data
        }

        fn get_left(&self) -> Option<Rc<RefCell<SetAVLTreeNode<i32, i32>>>> {
            self.as_ref().unwrap().get_left()
        }

        fn get_right(&self) -> Option<Rc<RefCell<SetAVLTreeNode<i32, i32>>>> {
            self.as_ref().unwrap().get_right()
        }
    }

    #[test]
    fn it_works() {
        let mut tree: SetAVLTree<i32, i32> = SetAVLTree::new();
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.root, None);

        println!("\n\n----------- insert 10 -----------");
        tree.insert(10);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.root.get_data(), 10);

        println!("\n\n----------- insert 20 -----------");
        tree.insert(20);
        assert_eq!(tree.len(), 2);
        assert_eq!(tree.root.get_data(), 10);
        assert_eq!(tree.root.get_right().get_data(), 20);

        println!("\n\n----------- insert 30 -----------");
        tree.insert(30);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.root.get_data(), 20);
        assert_eq!(tree.root.get_left().get_data(), 10);
        assert_eq!(tree.root.get_right().get_data(), 30);

        println!("\n\n----------- insert 40 -----------");
        tree.insert(40);
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.root.get_data(), 20);
        assert_eq!(tree.root.get_left().get_data(), 10);
        assert_eq!(tree.root.get_right().get_data(), 30);
        assert_eq!(tree.root.get_right().get_right().get_data(), 40);

        println!("\n\n----------- insert 50 -----------");
        tree.insert(50);
        assert_eq!(tree.len(), 5);
        assert_eq!(tree.root.get_data(), 20);
        assert_eq!(tree.root.get_left().get_data(), 10);
        assert_eq!(tree.root.get_right().get_data(), 40);
        assert_eq!(tree.root.get_right().get_left().get_data(), 30);
        assert_eq!(tree.root.get_right().get_right().get_data(), 50);

        println!("\n\n----------- insert 32 -----------");
        tree.insert(32);
        assert_eq!(tree.len(), 6);
        assert_eq!(tree.root.get_data(), 30);
        assert_eq!(tree.root.get_left().get_data(), 20);
        assert_eq!(tree.root.get_left().get_left().get_data(), 10);
        assert_eq!(tree.root.get_right().get_data(), 40);
        assert_eq!(tree.root.get_right().get_left().get_data(), 32);
        assert_eq!(tree.root.get_right().get_right().get_data(), 50);

        println!("\n\n----------- insert -42 -----------");
        tree.insert(-42);
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.root.get_data(), 30);
        assert_eq!(tree.root.get_left().get_data(), 10);
        assert_eq!(tree.root.get_left().get_left().get_data(), -42);
        assert_eq!(tree.root.get_left().get_right().get_data(), 20);
        assert_eq!(tree.root.get_right().get_data(), 40);
        assert_eq!(tree.root.get_right().get_left().get_data(), 32);
        assert_eq!(tree.root.get_right().get_right().get_data(), 50);

        println!("\n\n----------- insert -6 -----------");
        tree.insert(-6);
        assert_eq!(tree.len(), 8);
        assert_eq!(tree.root.get_data(), 30);
        assert_eq!(tree.root.get_left().get_data(), 10);
        assert_eq!(tree.root.get_left().get_left().get_data(), -42);
        assert_eq!(tree.root.get_left().get_left().get_right().get_data(), -6);
        assert_eq!(tree.root.get_left().get_right().get_data(), 20);
        assert_eq!(tree.root.get_right().get_data(), 40);
        assert_eq!(tree.root.get_right().get_left().get_data(), 32);
        assert_eq!(tree.root.get_right().get_right().get_data(), 50);

        println!("\n\n----------- insert -8 -----------");
        tree.insert(-8);
        assert_eq!(tree.len(), 9);
        assert_eq!(tree.root.get_data(), 30);
        assert_eq!(tree.root.get_left().get_data(), 10);
        assert_eq!(tree.root.get_left().get_left().get_data(), -8);
        assert_eq!(tree.root.get_left().get_left().get_left().get_data(), -42);
        assert_eq!(tree.root.get_left().get_left().get_right().get_data(), -6);
        assert_eq!(tree.root.get_left().get_right().get_data(), 20);
        assert_eq!(tree.root.get_right().get_data(), 40);
        assert_eq!(tree.root.get_right().get_left().get_data(), 32);
        assert_eq!(tree.root.get_right().get_right().get_data(), 50);

        println!("\n\n----------- delete 30 -----------");
        tree.delete(&30);
        assert_eq!(tree.len(), 8);
        assert_eq!(tree.root.get_data(), 20);
        assert_eq!(tree.root.get_left().get_data(), -8);
        assert_eq!(tree.root.get_left().get_left().get_data(), -42);
        assert_eq!(tree.root.get_left().get_right().get_data(), 10);
        assert_eq!(tree.root.get_left().get_right().get_left().get_data(), -6);
        assert_eq!(tree.root.get_right().get_data(), 40);
        assert_eq!(tree.root.get_right().get_left().get_data(), 32);
        assert_eq!(tree.root.get_right().get_right().get_data(), 50);

        println!("\n\n----------- delete -8 -----------");
        tree.delete(&-8);
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.root.get_data(), 20);
        assert_eq!(tree.root.get_left().get_data(), -6);
        assert_eq!(tree.root.get_left().get_left().get_data(), -42);
        assert_eq!(tree.root.get_left().get_right().get_data(), 10);
        assert_eq!(tree.root.get_right().get_data(), 40);
        assert_eq!(tree.root.get_right().get_left().get_data(), 32);
        assert_eq!(tree.root.get_right().get_right().get_data(), 50);

        println!("\n\n----------- delete 40 -----------");
        tree.delete(&40);
        assert_eq!(tree.len(), 6);
        assert_eq!(tree.root.get_data(), 20);
        assert_eq!(tree.root.get_left().get_data(), -6);
        assert_eq!(tree.root.get_left().get_left().get_data(), -42);
        assert_eq!(tree.root.get_left().get_right().get_data(), 10);
        assert_eq!(tree.root.get_right().get_data(), 32);
        assert_eq!(tree.root.get_right().get_right().get_data(), 50);

        println!("\n\n----------- delete 20 -----------");
        tree.delete(&20);
        assert_eq!(tree.len(), 5);
        assert_eq!(tree.root.get_data(), 10);
        assert_eq!(tree.root.get_left().get_data(), -6);
        assert_eq!(tree.root.get_left().get_left().get_data(), -42);
        assert_eq!(tree.root.get_right().get_data(), 32);
        assert_eq!(tree.root.get_right().get_right().get_data(), 50);

        println!("\n\n----------- delete 50 -----------");
        tree.delete(&50);
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.root.get_data(), 10);
        assert_eq!(tree.root.get_left().get_data(), -6);
        assert_eq!(tree.root.get_left().get_left().get_data(), -42);
        assert_eq!(tree.root.get_right().get_data(), 32);

        println!("\n\n----------- delete 32 -----------");
        tree.delete(&32);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.root.get_data(), -6);
        assert_eq!(tree.root.get_left().get_data(), -42);
        assert_eq!(tree.root.get_right().get_data(), 10);

        println!("\n\n----------- delete -6 -----------");
        tree.delete(&-6);
        assert_eq!(tree.len(), 2);
        assert_eq!(tree.root.get_data(), -42);
        assert_eq!(tree.root.get_right().get_data(), 10);

        println!("\n\n----------- delete -42 -----------");
        tree.delete(&-42);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.root.get_data(), 10);

        println!("\n\n----------- delete -10 -----------");
        tree.delete(&10);
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.root, None);
    }
}
