// red-black properties:
// 1. each node is either red or black
// 2. the root is black
// 3. every leaf (NIL) is black
// 4. if node is red, then both its children are black
// 5. for each node, all simple paths from the node to descendant leaves contain the
//    same number of black nodes

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::Key;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Color {
    Black,
    Red,
}

#[derive(Debug)]
struct RBTreeNode<T, K>
where
    T: Key<K> + PartialEq,
    K: Ord,
{
    data: T,
    color: Color,
    parent: Option<Weak<RefCell<RBTreeNode<T, K>>>>,
    left: Option<Rc<RefCell<RBTreeNode<T, K>>>>,
    right: Option<Rc<RefCell<RBTreeNode<T, K>>>>,
}

impl<T, K> PartialEq for RBTreeNode<T, K>
where
    T: Key<K> + PartialEq,
    K: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.color == other.color
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

impl<T, K> Key<K> for RBTreeNode<T, K>
where
    T: Key<K> + PartialEq,
    K: Ord,
{
    fn key(&self) -> &K {
        self.data.key()
    }
}

impl<T, K> RBTreeNode<T, K>
where
    T: Key<K> + PartialEq,
    K: Ord,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            color: Color::Red,
            parent: None,
            left: None,
            right: None,
        }
    }
}

trait RBTreeNodePointer<T, K>
where
    T: Key<K> + PartialEq,
    K: Ord,
{
    fn is_left_child(&self) -> bool;
    fn is_right_child(&self) -> bool;
    fn is_leaf(&self) -> bool;
    fn get_parent(&self) -> Option<Rc<RefCell<RBTreeNode<T, K>>>>;
    fn get_left(&self) -> Option<Rc<RefCell<RBTreeNode<T, K>>>>;
    fn get_right(&self) -> Option<Rc<RefCell<RBTreeNode<T, K>>>>;
    fn unlink_parent(&self);
    fn unlink_left(&self);
    fn unlink_right(&self);
    fn set_color(&self, color: Color);
    fn set_left(&self, left: &Rc<RefCell<RBTreeNode<T, K>>>);
    fn set_right(&self, right: &Rc<RefCell<RBTreeNode<T, K>>>);
}

impl<T, K> RBTreeNodePointer<T, K> for Rc<RefCell<RBTreeNode<T, K>>>
where
    T: Key<K> + PartialEq,
    K: Ord,
{
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
        self.borrow().left.is_none() && self.borrow().right.is_none()
    }

    fn get_parent(&self) -> Option<Rc<RefCell<RBTreeNode<T, K>>>> {
        match self.borrow().parent {
            Some(_) => Some(self.borrow().parent.as_ref().unwrap().upgrade().unwrap()),
            None => None,
        }
    }

    fn get_left(&self) -> Option<Rc<RefCell<RBTreeNode<T, K>>>> {
        match self.borrow().left {
            Some(_) => Some(Rc::clone(self.borrow().left.as_ref().unwrap())),
            None => None,
        }
    }

    fn get_right(&self) -> Option<Rc<RefCell<RBTreeNode<T, K>>>> {
        match self.borrow().right {
            Some(_) => Some(Rc::clone(self.borrow().right.as_ref().unwrap())),
            None => None,
        }
    }

    fn unlink_parent(&self) {
        if self.is_left_child() {
            self.get_parent().unwrap().borrow_mut().left.take();
        } else if self.is_right_child() {
            self.get_parent().unwrap().borrow_mut().right.take();
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

    fn set_color(&self, color: Color) {
        self.borrow_mut().color = color;
    }

    fn set_left(&self, left: &Rc<RefCell<RBTreeNode<T, K>>>) {
        let left = Rc::clone(left);

        self.unlink_left();
        self.borrow_mut().left = Some(Rc::clone(&left));

        left.unlink_parent();
        left.borrow_mut().parent = Some(Rc::downgrade(self));
    }

    fn set_right(&self, right: &Rc<RefCell<RBTreeNode<T, K>>>) {
        let right = Rc::clone(right);

        self.unlink_right();
        self.borrow_mut().right = Some(Rc::clone(&right));

        right.unlink_parent();
        right.borrow_mut().parent = Some(Rc::downgrade(self));
    }
}

trait RBTreeNodePointerColorHelper {
    fn get_color(&self) -> Color;
}

impl<T, K> RBTreeNodePointerColorHelper for Option<Rc<RefCell<RBTreeNode<T, K>>>>
where
    T: Key<K> + PartialEq,
    K: Ord,
{
    // Red-Black Tree expects every node's pointer pointing to either to some other node
    // or the Nil node. And Red-Black Tree says that the Nil node is black. So this helper
    // function trick the None type as black color node.
    fn get_color(&self) -> Color {
        match self {
            Some(x) => x.borrow().color,
            None => Color::Black,
        }
    }
}

pub struct RBTree<T, K>
where
    T: Key<K> + PartialEq,
    K: Ord,
{
    root: Option<Rc<RefCell<RBTreeNode<T, K>>>>,
}

impl<T, K> RBTree<T, K>
where
    T: Key<K> + PartialEq,
    K: Ord,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    fn set_root(&mut self, root: &Rc<RefCell<RBTreeNode<T, K>>>) {
        root.unlink_parent();
        self.root = Some(Rc::clone(&root));
    }

    pub fn insert(&mut self, data: T) {
        let z = Rc::new(RefCell::new(RBTreeNode::new(data)));
        if self.root.is_none() {
            z.set_color(Color::Black);
            self.root = Some(z);
            return;
        }

        let mut x = Some(Rc::clone(self.root.as_ref().unwrap()));
        let mut y: Option<Rc<RefCell<RBTreeNode<T, K>>>> = None;
        let mut should_insert_to_left_of_y = false;
        while x.is_some() {
            y = x;
            let y = y.as_ref().unwrap();
            match z.borrow().key() < y.borrow().key() {
                true => {
                    x = y.get_left();
                    should_insert_to_left_of_y = true;
                }
                false => {
                    x = y.get_right();
                    should_insert_to_left_of_y = false;
                }
            }
        }

        let y = y.as_ref().unwrap();
        match should_insert_to_left_of_y {
            true => y.set_left(&z),
            false => y.set_right(&z),
        };

        self.insert_fixup(&z);
    }

    fn insert_fixup(&mut self, z: &Rc<RefCell<RBTreeNode<T, K>>>) {
        let get_p_and_pp = |node: &Rc<RefCell<RBTreeNode<T, K>>>| {
            (
                node.get_parent().unwrap(),
                node.get_parent().unwrap().get_parent().unwrap(),
            )
        };

        let mut z = Rc::clone(z);
        while z.get_parent().get_color() == Color::Red {
            // z.p is guaranteed to be Some(...) since its color is red
            // z.p.p is also guaranteed to be Some(...) since z.p.color is red
            // red-black properties #2 "the root is black"
            let (mut z_p, mut z_p_p) = get_p_and_pp(&z);
            match z_p.is_left_child() {
                true => match z_p_p.get_right().get_color() {
                    Color::Red => {
                        //           __z.p.p__        Coloring                __Red__
                        //      __Red         Red     -------->        __Black       Black
                        // Red z                                  Red z
                        //
                        //                              And
                        //
                        //       ______z.p.p__        Coloring             ______Red__
                        //    Red__           Red     -------->     Black__           Black
                        //         Red z                                   Red z
                        z_p.set_color(Color::Black);
                        z_p_p.get_right().unwrap().set_color(Color::Black);
                        z_p_p.set_color(Color::Red);
                        z = z_p_p;
                    }
                    Color::Black => {
                        //           __z.p.p__                      ______z.p.p__
                        //      __Red         Black     Or     Red__             Black
                        // Red z                                    Red z
                        if z.is_right_child() {
                            //      ______z.p.p__          Left Rotate              __z.p.p__
                            // Red__             Black     ----------->        __Red         Black
                            //      Red z                                 Red z
                            z = z_p;
                            self.left_rotate(&z);
                            (z_p, z_p_p) = get_p_and_pp(&z);
                        }
                        //           __z.p.p__         Coloring                __Red__         Right Rotate            __Black__
                        //      __Red         Black    -------->        __Black       Black    ------------->     Red z         Red
                        // Red z                                   Red z
                        z_p.set_color(Color::Black);
                        z_p_p.set_color(Color::Red);
                        self.right_rotate(&z_p_p);
                    }
                },
                false => match z_p_p.get_left().get_color() {
                    Color::Red => {
                        //    __z.p.p__              Coloring              __Red__
                        // Red         Red__         -------->        Black       Black__
                        //                  Red z                                        Red z
                        //
                        //                              And
                        //
                        //       __z.p.p_________        Coloring           __Red_________
                        //    Red              __Red     -------->     Black            __Black
                        //                Red z                                    Red z
                        z_p.set_color(Color::Black);
                        z_p_p.get_left().unwrap().set_color(Color::Black);
                        z_p_p.set_color(Color::Red);
                        z = z_p_p;
                    }
                    Color::Black => {
                        //           __z.p.p__                          __z.p.p_________
                        //      Black         Red__         Or     Black                __Red
                        //                         Red z                           Red z
                        if z.is_left_child() {
                            //         __z.p.p_________        Right Rotate            __z.p.p__
                            //  Black                __Red     ----------->       Black         Red__
                            //                  Red z                                                Red z
                            z = z_p;
                            self.right_rotate(&z);
                            (z_p, z_p_p) = get_p_and_pp(&z);
                        }
                        //       __z.p.p__               Coloring             __Red__                Left Rotate          __Black__
                        //  Black         Red__          -------->       Black       Black__         ----------->      Red         Red z
                        //                     Red z                                        Red z
                        z_p.set_color(Color::Black);
                        z_p_p.set_color(Color::Red);
                        self.left_rotate(&z_p_p);
                    }
                },
            }
        }
        self.root.as_ref().unwrap().set_color(Color::Black);
    }

    //   __x__                 __y__
    //  a   __y__     ->    __x__   c
    //     b     c         a     b
    fn left_rotate(&mut self, x: &Rc<RefCell<RBTreeNode<T, K>>>) {
        let y = x.get_right();
        assert!(y.is_some(), "can't do left rotate if x.right is None");
        let y = y.unwrap();

        if let Some(ref b) = y.get_left() {
            x.set_right(b);
        }

        // handle x's parent
        match x == self.root.as_ref().unwrap() {
            true => self.set_root(&y),
            false => {
                let p = x.get_parent().unwrap();
                match x.is_left_child() {
                    true => p.set_left(&y),
                    false => p.set_right(&y),
                }
            }
        }
        y.set_left(&x);
    }

    //      __y__             __x__
    //   __x__   c     ->    a   __y__
    //  a     b                 b     c
    fn right_rotate(&mut self, y: &Rc<RefCell<RBTreeNode<T, K>>>) {
        let x = y.get_left();
        assert!(x.is_some(), "can't do right rotate if y.left is None");
        let x = x.unwrap();

        if let Some(ref b) = x.get_right() {
            y.set_left(b);
        }

        // handle y's parent
        match y == self.root.as_ref().unwrap() {
            true => self.set_root(&x),
            false => {
                let p = y.get_parent().unwrap();
                match y.is_left_child() {
                    true => p.set_left(&x),
                    false => p.set_right(&x),
                }
            }
        }
        x.set_right(&y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Item {
        name: String,
        id: i32,
    }

    impl Key<i32> for Item {
        fn key(&self) -> &i32 {
            &self.id
        }
    }

    impl Item {
        pub fn new(name: &str, id: i32) -> Self {
            Self {
                name: name.to_owned(),
                id,
            }
        }
    }

    macro_rules! assert_node {
        ($node:expr, $name:expr) => {
            assert_eq!($node.as_ref().unwrap().borrow().data.name, $name);
        };
    }

    fn _in_order_collect(node: Rc<RefCell<RBTreeNode<Item, i32>>>, res: &mut Vec<(String, Color)>) {
        if let Some(left) = node.get_left() {
            _in_order_collect(left, res)
        }
        res.push((node.borrow().data.name.to_string(), node.borrow().color));
        if let Some(right) = node.get_right() {
            _in_order_collect(right, res)
        }
    }

    fn in_order_collect(node: Rc<RefCell<RBTreeNode<Item, i32>>>) -> Vec<(String, Color)> {
        let mut res = vec![];
        _in_order_collect(node, &mut res);
        res
    }

    macro_rules! assert_tree_in_order {
        ($tree:expr, $expect:expr) => {{
            let collected = in_order_collect(Rc::clone($tree.root.as_ref().unwrap()));
            assert_eq!(collected.len(), $expect.len());
            for i in 0..collected.len() {
                assert_eq!(collected[i].0, $expect[i].0);
                assert_eq!(collected[i].1, $expect[i].1);
            }
        }};
    }

    #[test]
    fn test_rotate_left_on_root() {
        //   __x__                 __y__
        //  a   __y__     ->    __x__   c
        //     b     c         a     b
        let x = Rc::new(RefCell::new(RBTreeNode::new(Item::new("x", 0))));
        let y = Rc::new(RefCell::new(RBTreeNode::new(Item::new("y", 0))));
        let a = Rc::new(RefCell::new(RBTreeNode::new(Item::new("a", 0))));
        let b = Rc::new(RefCell::new(RBTreeNode::new(Item::new("b", 0))));
        let c = Rc::new(RefCell::new(RBTreeNode::new(Item::new("c", 0))));
        x.set_left(&a);
        x.set_right(&y);
        y.set_left(&b);
        y.set_right(&c);
        let mut tree: RBTree<Item, i32> = RBTree::new();
        tree.set_root(&x);
        tree.left_rotate(&x);
        assert_node!(tree.root, "y");
        assert_node!(y.get_left(), "x");
        assert_node!(y.get_right(), "c");
        assert_node!(x.get_left(), "a");
        assert_node!(x.get_right(), "b");
        assert_node!(x.get_parent(), "y");
        assert_node!(a.get_parent(), "x");
        assert_node!(b.get_parent(), "x");
        assert_node!(c.get_parent(), "y");
    }

    #[test]
    fn test_rotate_left() {
        //   __x__                 __y__
        //  a   __y__     ->    __x__   c
        //     b     c         a     b
        let root = Rc::new(RefCell::new(RBTreeNode::new(Item::new("root", 0))));
        let x = Rc::new(RefCell::new(RBTreeNode::new(Item::new("x", 0))));
        let y = Rc::new(RefCell::new(RBTreeNode::new(Item::new("y", 0))));
        let a = Rc::new(RefCell::new(RBTreeNode::new(Item::new("a", 0))));
        let b = Rc::new(RefCell::new(RBTreeNode::new(Item::new("b", 0))));
        let c = Rc::new(RefCell::new(RBTreeNode::new(Item::new("c", 0))));
        root.set_left(&x);
        x.set_left(&a);
        x.set_right(&y);
        y.set_left(&b);
        y.set_right(&c);
        let mut tree: RBTree<Item, i32> = RBTree::new();
        tree.set_root(&root);
        tree.left_rotate(&x);
        assert_node!(tree.root, "root");
        assert_node!(root.get_left(), "y");
        assert_node!(y.get_parent(), "root");
        assert_node!(y.get_left(), "x");
        assert_node!(y.get_right(), "c");
        assert_node!(x.get_parent(), "y");
        assert_node!(x.get_left(), "a");
        assert_node!(x.get_right(), "b");
        assert_node!(a.get_parent(), "x");
        assert_node!(b.get_parent(), "x");
        assert_node!(c.get_parent(), "y");
    }

    #[test]
    fn test_rotate_right_on_root() {
        //      __y__             __x__
        //   __x__   c     ->    a   __y__
        //  a     b                 b     c
        let x = Rc::new(RefCell::new(RBTreeNode::new(Item::new("x", 0))));
        let y = Rc::new(RefCell::new(RBTreeNode::new(Item::new("y", 0))));
        let a = Rc::new(RefCell::new(RBTreeNode::new(Item::new("a", 0))));
        let b = Rc::new(RefCell::new(RBTreeNode::new(Item::new("b", 0))));
        let c = Rc::new(RefCell::new(RBTreeNode::new(Item::new("c", 0))));
        x.set_left(&a);
        x.set_right(&b);
        y.set_left(&x);
        y.set_right(&c);
        let mut tree: RBTree<Item, i32> = RBTree::new();
        tree.set_root(&y);
        tree.right_rotate(&y);
        assert_node!(tree.root, "x");
        assert_node!(x.get_left(), "a");
        assert_node!(x.get_right(), "y");
        assert_node!(y.get_left(), "b");
        assert_node!(y.get_right(), "c");
        assert_node!(y.get_parent(), "x");
        assert_node!(a.get_parent(), "x");
        assert_node!(b.get_parent(), "y");
        assert_node!(c.get_parent(), "y");
    }

    #[test]
    fn test_rotate_right() {
        //      __y__             __x__
        //   __x__   c     ->    a   __y__
        //  a     b                 b     c
        let root = Rc::new(RefCell::new(RBTreeNode::new(Item::new("root", 0))));
        let x = Rc::new(RefCell::new(RBTreeNode::new(Item::new("x", 0))));
        let y = Rc::new(RefCell::new(RBTreeNode::new(Item::new("y", 0))));
        let a = Rc::new(RefCell::new(RBTreeNode::new(Item::new("a", 0))));
        let b = Rc::new(RefCell::new(RBTreeNode::new(Item::new("b", 0))));
        let c = Rc::new(RefCell::new(RBTreeNode::new(Item::new("c", 0))));
        root.set_left(&y);
        y.set_left(&x);
        y.set_right(&c);
        x.set_left(&a);
        x.set_right(&b);
        let mut tree: RBTree<Item, i32> = RBTree::new();
        tree.set_root(&root);
        tree.right_rotate(&y);
        assert_node!(tree.root, "root");
        assert_node!(root.get_left(), "x");
        assert_node!(x.get_parent(), "root");
        assert_node!(x.get_left(), "a");
        assert_node!(x.get_right(), "y");
        assert_node!(y.get_parent(), "x");
        assert_node!(y.get_left(), "b");
        assert_node!(y.get_right(), "c");
        assert_node!(a.get_parent(), "x");
        assert_node!(b.get_parent(), "y");
        assert_node!(c.get_parent(), "y");
    }

    #[test]
    fn test_insert() {
        let mut tree: RBTree<Item, i32> = RBTree::new();
        tree.insert(Item::new("1", 1));
        assert_tree_in_order!(&tree, [("1", Color::Black)]);

        tree.insert(Item::new("2", 2));
        assert_tree_in_order!(&tree, [("1", Color::Black), ("2", Color::Red)]);

        tree.insert(Item::new("3", 3));
        assert_tree_in_order!(
            &tree,
            [("1", Color::Red), ("2", Color::Black), ("3", Color::Red)]
        );

        tree.insert(Item::new("4", 4));
        assert_tree_in_order!(
            &tree,
            [
                ("1", Color::Black),
                ("2", Color::Black),
                ("3", Color::Black),
                ("4", Color::Red)
            ]
        );

        tree.insert(Item::new("5", 5));
        assert_tree_in_order!(
            &tree,
            [
                ("1", Color::Black),
                ("2", Color::Black),
                ("3", Color::Red),
                ("4", Color::Black),
                ("5", Color::Red)
            ]
        );

        tree.insert(Item::new("6", 6));
        assert_tree_in_order!(
            &tree,
            [
                ("1", Color::Black),
                ("2", Color::Black),
                ("3", Color::Black),
                ("4", Color::Red),
                ("5", Color::Black),
                ("6", Color::Red)
            ]
        );

        tree.insert(Item::new("7", 7));
        assert_tree_in_order!(
            &tree,
            [
                ("1", Color::Black),
                ("2", Color::Black),
                ("3", Color::Black),
                ("4", Color::Red),
                ("5", Color::Red),
                ("6", Color::Black),
                ("7", Color::Red)
            ]
        );

        tree.insert(Item::new("8", 8));
        assert_tree_in_order!(
            &tree,
            [
                ("1", Color::Black),
                ("2", Color::Red),
                ("3", Color::Black),
                ("4", Color::Black),
                ("5", Color::Black),
                ("6", Color::Red),
                ("7", Color::Black),
                ("8", Color::Red)
            ]
        );

        tree.insert(Item::new("9", 9));
        assert_tree_in_order!(
            &tree,
            [
                ("1", Color::Black),
                ("2", Color::Red),
                ("3", Color::Black),
                ("4", Color::Black),
                ("5", Color::Black),
                ("6", Color::Red),
                ("7", Color::Red),
                ("8", Color::Black),
                ("9", Color::Red),
            ]
        );

        tree.insert(Item::new("10", 10));
        assert_tree_in_order!(
            &tree,
            [
                ("1", Color::Black),
                ("2", Color::Black),
                ("3", Color::Black),
                ("4", Color::Black),
                ("5", Color::Black),
                ("6", Color::Black),
                ("7", Color::Black),
                ("8", Color::Red),
                ("9", Color::Black),
                ("10", Color::Red)
            ]
        );

        tree.insert(Item::new("11", 11));
        assert_tree_in_order!(
            &tree,
            [
                ("1", Color::Black),
                ("2", Color::Black),
                ("3", Color::Black),
                ("4", Color::Black),
                ("5", Color::Black),
                ("6", Color::Black),
                ("7", Color::Black),
                ("8", Color::Red),
                ("9", Color::Red),
                ("10", Color::Black),
                ("11", Color::Red)
            ]
        );

        tree.insert(Item::new("12", 12));
        assert_tree_in_order!(
            &tree,
            [
                ("1", Color::Black),
                ("2", Color::Black),
                ("3", Color::Black),
                ("4", Color::Black),
                ("5", Color::Black),
                ("6", Color::Red),
                ("7", Color::Black),
                ("8", Color::Black),
                ("9", Color::Black),
                ("10", Color::Red),
                ("11", Color::Black),
                ("12", Color::Red)
            ]
        );
        assert_node!(tree.root, "4");
    }
}
