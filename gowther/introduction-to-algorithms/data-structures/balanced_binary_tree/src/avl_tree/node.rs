// AVL properties:
// 1. every node is height-balanced
//    the left and right subtrees of a height-balanced node differ in height by at most 1

use std::cmp::max;

pub trait Pointer: Sized + PartialEq {
    fn clone_pointer(&self) -> Self;

    fn get_parent(&self) -> Option<Self>;
    fn get_left(&self) -> Option<Self>;
    fn get_right(&self) -> Option<Self>;

    fn set_left(&self, left: &Self) -> Option<Self>;
    fn set_right(&self, right: &Self) -> Option<Self>;

    fn take_parent(&self) -> Option<Self>;
    fn take_left(&self) -> Option<Self>;
    fn take_right(&self) -> Option<Self>;

    fn unlink_parent(&self) -> Option<Self> {
        match self.get_parent() {
            Some(parent) => {
                match self.is_left_child() {
                    true => parent.take_left(),
                    false => parent.take_right(),
                };
                self.take_parent()
            }
            None => None,
        }
    }
    fn unlink_left(&self) -> Option<Self> {
        match self.get_left() {
            Some(left) => {
                left.unlink_parent();
                Some(left)
            }
            None => None,
        }
    }
    fn unlink_right(&self) -> Option<Self> {
        match self.get_right() {
            Some(right) => {
                right.unlink_parent();
                Some(right)
            }
            None => None,
        }
    }

    fn is_left_child(&self) -> bool {
        match self.get_parent() {
            Some(parent) => match parent.get_left() {
                Some(left) => self == &left,
                None => false,
            },
            None => false,
        }
    }
    fn is_right_child(&self) -> bool {
        match self.get_parent() {
            Some(parent) => match parent.get_right() {
                Some(right) => self == &right,
                None => false,
            },
            None => false,
        }
    }
    fn is_leaf(&self) -> bool {
        self.get_left().is_some() && self.get_right().is_some()
    }
}

pub trait AVLTreeNode: Sized + Pointer + std::fmt::Debug {
    fn get_height(x: Option<Self>) -> i32;
    fn set_height(&self, height: i32);
    fn swap_item(x: &Self, y: &Self);
    fn subtree_update(&self) {
        self.set_height(
            1 + max(
                Self::get_height(self.get_left()),
                Self::get_height(self.get_right()),
            ),
        )
    }
    fn skew(&self) -> i32 {
        Self::get_height(self.get_right()) - Self::get_height(self.get_left())
    }
    fn subtree_first(&self) -> Self {
        match self.get_left() {
            Some(left) => left.subtree_first(),
            None => self.clone_pointer(),
        }
    }
    fn subtree_last(&self) -> Self {
        match self.get_right() {
            Some(left) => left.subtree_last(),
            None => self.clone_pointer(),
        }
    }
    fn successor(&self) -> Option<Self> {
        match self.get_right() {
            Some(right) => Some(right.subtree_first()),
            None => {
                let mut x = self.clone_pointer();
                loop {
                    match x.get_parent() {
                        Some(parent) => match x.is_left_child() {
                            true => return Some(parent),
                            false => x = parent,
                        },
                        None => return None,
                    }
                }
            }
        }
    }
    fn predecessor(&self) -> Option<Self> {
        match self.get_left() {
            Some(left) => Some(left.subtree_last()),
            None => {
                let mut x = self.clone_pointer();
                loop {
                    match x.get_parent() {
                        Some(parent) => match x.is_right_child() {
                            true => return Some(parent),
                            false => x = parent,
                        },
                        None => return None,
                    }
                }
            }
        }
    }
    fn subtree_insert_before(&self, x: Self) {
        match self.get_left() {
            Some(left) => {
                let left_last = left.subtree_last();
                left_last.set_right(&x);
                left_last.maintain();
            }
            None => {
                self.set_left(&x);
                self.maintain();
            }
        }
    }
    fn subtree_insert_after(&self, x: Self) {
        match self.get_right() {
            Some(right) => {
                let right_last = right.subtree_last();
                right_last.set_left(&x);
                right_last.maintain();
            }
            None => {
                self.set_right(&x);
                self.maintain();
            }
        }
    }
    fn subtree_delete(&self) {
        match (self.get_left(), self.get_right()) {
            (None, None) => {
                if let Some(parent) = self.unlink_parent() {
                    parent.maintain();
                }
            }
            (Some(left), _) => {
                let successor = left.subtree_last();
                Self::swap_item(&self, &successor);
                successor.subtree_delete();
            }
            (None, Some(right)) => {
                let predecessor = right.subtree_first();
                Self::swap_item(&self, &predecessor);
                predecessor.subtree_delete();
            }
        }
    }
    //      _____d__             __b_____
    //   __a__      e     ->    a      __d__
    //  b     c                       c     e
    fn subtree_rotate_right(&self) {
        let _d = self.clone_pointer();
        let _b = _d.unlink_left().unwrap();
        let e = _d.unlink_right();
        let a = _b.unlink_left();
        let c = _b.unlink_right();

        let b = _d;
        let d = _b;
        Self::swap_item(&b, &d);

        if let Some(c) = c {
            d.set_left(&c);
        }
        if let Some(e) = e {
            d.set_right(&e);
        }
        d.subtree_update();

        if let Some(a) = a {
            b.set_left(&a);
        }
        b.set_right(&d);
        b.subtree_update();
    }
    //   __b_____                 _____d__
    //  a      __d__     ->    __b__      e
    //        c     e         a     c
    fn subtree_rotate_left(&self) {
        let _b = self.clone_pointer();
        let a = _b.unlink_left();
        let _d = _b.unlink_right().unwrap();
        let c = _d.unlink_left();
        let e = _d.unlink_right();

        let b = _d;
        let d = _b;
        Self::swap_item(&b, &d);

        if let Some(a) = a {
            b.set_left(&a);
        }
        if let Some(c) = c {
            b.set_left(&c);
        }
        b.subtree_update();

        d.set_left(&b);
        if let Some(e) = e {
            d.set_right(&e);
        }
        d.subtree_update();
    }
    fn re_balance(&self) {
        if self.skew() == 2 {
            let right = self
                .get_right()
                .expect("skew == 2 means right child must exist");
            if right.skew() == -1 {
                right.subtree_rotate_right();
            }
            self.subtree_rotate_left();
        } else if self.skew() == -2 {
            let left = self
                .get_left()
                .expect("skew == -2 means left child must exist");
            if left.skew() == 1 {
                left.subtree_rotate_left();
            }
            self.subtree_rotate_right();
        }
    }
    fn maintain(&self) {
        self.re_balance();
        self.subtree_update();
        if let Some(parent) = self.get_parent() {
            parent.maintain();
        }
    }
}
