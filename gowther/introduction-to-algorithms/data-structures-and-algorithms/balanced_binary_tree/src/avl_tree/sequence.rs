use super::node::{AVLTreeNode, Pointer};

use std::{
    cell::RefCell,
    mem::swap,
    rc::{Rc, Weak},
};

trait Sequence {}

// need to maintain a new subtree property: size
struct SequenceAVLTreeNode<T, K> {
    data: T,

    // pointers
    parent: Option<Weak<RefCell<SequenceAVLTreeNode<T, K>>>>,
    left: Option<Rc<RefCell<SequenceAVLTreeNode<T, K>>>>,
    right: Option<Rc<RefCell<SequenceAVLTreeNode<T, K>>>>,

    // subtree properties
    height: usize,
    size: usize,
}
