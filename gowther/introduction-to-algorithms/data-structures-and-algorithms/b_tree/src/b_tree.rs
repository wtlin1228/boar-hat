// B-Tree Properties:
// 1. Every node x has the following attributes:
//     a. x.n, the number of keys currently stored in the node x.
//     b. the x.n keys themselves, x.key_1, x.key_2, ..., x.key_x.n.
//     c. x.leaf, a boolean value that is TRUE if x is a leaf and FALSE if x is an internal node.
// 2. Each internal node x also contains x.n + 1 pointers x.c_1, x.c_2, ..., x.c_n+1 to its
//    children. Leaf nodes have no children, and so their c_i attributes are undefined.
// 3. The keys x.key_i separate the ranges of keys stored in each subtree:
//    if k_i is any key stored in the subtree with root x.c_i, then
//    k_1 <= x.key_1 <= k_2 <= x.key_2 <= ... <= x.key_x.n  <= k_x.n+1
// 4. All leaves have the same depth, which is the tree's height h.
// 5. Nodes have lower and upper bounds on the number of keys they can contain, expressed in terms
//    of a fixed integer t >=2 called the minimum degree of the B-tree:
//     a. Every node other than the root must have at least t - 1 keys. Every internal node other
//        than the root thus has at least t children. If the tree is nonempty, the root must have
//        at least one key.
//     b. Every node may contain at most 2t - 1 keys. Therefore, an internal node may have at most
//        2t children. We say that a node is full if it contains exactly 2t - 1 keys.

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

type Pointer<K, V> = Rc<RefCell<BTreeNode<K, V>>>;
type WeakPointer<K, V> = Weak<RefCell<BTreeNode<K, V>>>;

#[derive(Debug)]
pub struct BTreeNode<K, V> {
    keys: Vec<K>,
    vals: Vec<V>,
    parent: Option<WeakPointer<K, V>>,
    children: Option<Vec<Pointer<K, V>>>,
    leaf: bool,
}

impl<K, V> BTreeNode<K, V> {
    fn new(t: usize) -> Self {
        Self {
            keys: Vec::with_capacity(t * 2 - 1),
            vals: Vec::with_capacity(t * 2 - 1),
            parent: None,
            children: Some(Vec::with_capacity(t * 2)),
            leaf: false,
        }
    }
}

#[derive(Debug)]
pub struct BTree<K, V> {
    root: Option<Pointer<K, V>>,
    t: usize,
}

impl<K, V> BTree<K, V>
where
    K: Ord,
{
    fn allocate_node(&self) -> Pointer<K, V> {
        Rc::new(RefCell::new(BTreeNode::new(self.t)))
    }

    pub fn new(t: usize) -> Self {
        let mut tree = Self { root: None, t };
        let x = tree.allocate_node();
        x.borrow_mut().leaf = true;
        tree.root = Some(x);
        tree
    }

    pub fn search(x: &Pointer<K, V>, k: &K) -> Option<(Pointer<K, V>, usize)> {
        let leaf = x.borrow().leaf;
        let search_result = x.borrow().keys.binary_search(k);
        match search_result {
            Ok(i) => return Some((Rc::clone(&x), i)),
            Err(i) => match leaf {
                true => return None,
                false => {
                    // DISK-READ(x.c_i)
                    return Self::search(&x.borrow().children.as_ref().unwrap()[i], k);
                }
            },
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        let r = Rc::clone(self.root.as_ref().unwrap());
        let is_root_full = r.borrow().keys.len() == self.t * 2 - 1;
        match is_root_full {
            true => {
                let s = self.split_root();
                self.insert_non_full(&s, k, v)
            }
            false => self.insert_non_full(&r, k, v),
        }
    }

    // y = x.c_i
    //              ┌─────────┬─────┬─────────┐
    //  y.keys      │  t - 1  │  1  │  t - 1  │
    //              └─────────┴─────┴─────────┘
    //              ┌───────────┐ ┌───────────┐
    //  y.children  │     t     │ │     t     │
    //              └───────────┘ └───────────┘
    fn split_child(&self, x: &Pointer<K, V>, i: usize) {
        // full node to split
        let y = Rc::clone(&x.borrow().children.as_ref().unwrap()[i]);
        assert_eq!(y.borrow().keys.len(), self.t * 2 - 1);

        let z = self.allocate_node();
        z.borrow_mut().leaf = y.borrow().leaf;
        z.borrow_mut().parent = y.borrow().parent.clone();

        // z will take half of y
        let mut right_half_keys = y.borrow_mut().keys.split_off(self.t);
        z.borrow_mut().keys.append(&mut right_half_keys);
        let mut right_half_vals = y.borrow_mut().vals.split_off(self.t);
        z.borrow_mut().vals.append(&mut right_half_vals);
        if y.borrow().leaf == false {
            let mut right_half_children =
                y.borrow_mut().children.as_mut().unwrap().split_off(self.t);
            z.borrow_mut()
                .children
                .as_mut()
                .unwrap()
                .append(&mut right_half_children);
        }

        // shift x's keys, vals, children to the right
        x.borrow_mut()
            .keys
            .insert(i, y.borrow_mut().keys.pop().unwrap());
        x.borrow_mut()
            .vals
            .insert(i, y.borrow_mut().vals.pop().unwrap());
        x.borrow_mut().children.as_mut().unwrap().insert(i + 1, z);

        // DISK-WRITE(y)
        // DISK-WRITE(z)
        // DISK-WRITE(x)
    }

    fn split_root(&mut self) -> Pointer<K, V> {
        let s = self.allocate_node();
        let original_root = self.root.take().unwrap();
        original_root.borrow_mut().parent = Some(Rc::downgrade(&s));
        s.borrow_mut()
            .children
            .as_mut()
            .unwrap()
            .push(original_root);
        self.split_child(&s, 0);
        self.root = Some(Rc::clone(&s));
        s
    }

    fn insert_non_full(&self, x: &Pointer<K, V>, k: K, v: V) {
        let is_x_a_leaf = x.borrow().leaf;
        match is_x_a_leaf {
            true => {
                let search_result = x.borrow().keys.binary_search(&k);
                match search_result {
                    Ok(_) => unimplemented!("don't support same key insertion for now"),
                    Err(i) => {
                        x.borrow_mut().keys.insert(i, k);
                        x.borrow_mut().vals.insert(i, v);
                        // DISK-WRITE(x)
                    }
                };
            }
            false => {
                let search_result = x.borrow().keys.binary_search(&k);
                match search_result {
                    Ok(_) => unimplemented!("don't support same key insertion for now"),
                    Err(i) => {
                        // DISK-READ(x.c_i)
                        let is_child_node_full =
                            x.borrow().children.as_ref().unwrap()[i].borrow().keys.len()
                                == self.t * 2 - 1;
                        match is_child_node_full {
                            true => {
                                self.split_child(x, i);
                                let is_k_bigger_than_x_i = k > x.borrow().keys[i];
                                let mut i = i;
                                if is_k_bigger_than_x_i {
                                    i += 1;
                                }
                                self.insert_non_full(
                                    &x.borrow().children.as_ref().unwrap()[i],
                                    k,
                                    v,
                                );
                            }
                            false => {
                                self.insert_non_full(
                                    &x.borrow().children.as_ref().unwrap()[i],
                                    k,
                                    v,
                                );
                            }
                        }
                    }
                };
            }
        }
    }

    pub fn delete(&mut self, k: &K) -> Result<V, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_node {
        ($some_pointer:expr, $keys:expr, $vals:expr) => {
            assert_eq!($some_pointer.as_ref().unwrap().borrow().keys, $keys);
            assert_eq!($some_pointer.as_ref().unwrap().borrow().vals, $vals);
        };
    }

    macro_rules! assert_children {
        ($pointer:expr, $expected_children:expr) => {
            $expected_children
                .into_iter()
                .enumerate()
                .for_each(|(i, expect)| {
                    assert_node!(
                        $pointer.borrow().children.as_ref().unwrap().get(i),
                        expect,
                        expect
                    );
                });
        };
    }

    #[test]
    fn test_fundamental_types() {
        let t = 3;
        let tree: BTree<char, char> = BTree {
            t,
            root: Some(Rc::new(RefCell::new(BTreeNode {
                keys: Vec::with_capacity(t * 2 - 1),
                vals: Vec::with_capacity(t * 2 - 1),
                parent: None,
                children: Some(Vec::with_capacity(t * 2)),
                leaf: false,
            }))),
        };

        //          [__G_________M_________P_______________X__]
        //  [A_C_D_E]     [J_K]     [N_O]     [R_S_T_U_V]     [Y_Z]
        //
        let mut root = tree.root.as_ref().unwrap().borrow_mut();
        root.keys.append(&mut vec!['G', 'M', 'P', 'X']);
        root.vals.append(&mut vec!['G', 'M', 'P', 'X']);
        root.children.as_mut().unwrap().append(&mut vec![
            Rc::new(RefCell::new(BTreeNode {
                keys: vec!['A', 'C', 'D', 'E'],
                vals: vec!['A', 'C', 'D', 'E'],
                parent: Some(Rc::downgrade(tree.root.as_ref().unwrap())),
                children: None,
                leaf: true,
            })),
            Rc::new(RefCell::new(BTreeNode {
                keys: vec!['J', 'K'],
                vals: vec!['J', 'K'],
                parent: Some(Rc::downgrade(tree.root.as_ref().unwrap())),
                children: None,
                leaf: true,
            })),
            Rc::new(RefCell::new(BTreeNode {
                keys: vec!['N', 'O'],
                vals: vec!['N', 'O'],
                parent: Some(Rc::downgrade(tree.root.as_ref().unwrap())),
                children: None,
                leaf: true,
            })),
            Rc::new(RefCell::new(BTreeNode {
                keys: vec!['R', 'S', 'T', 'U', 'V'],
                vals: vec!['R', 'S', 'T', 'U', 'V'],
                parent: Some(Rc::downgrade(tree.root.as_ref().unwrap())),
                children: None,
                leaf: true,
            })),
            Rc::new(RefCell::new(BTreeNode {
                keys: vec!['Y', 'Z'],
                vals: vec!['Y', 'Z'],
                parent: Some(Rc::downgrade(tree.root.as_ref().unwrap())),
                children: None,
                leaf: true,
            })),
        ])
    }

    #[test]
    fn test_insert() {
        let mut tree = BTree::new(3);
        tree.insert('A', 'A');
        tree.insert('C', 'C');
        tree.insert('G', 'G');
        tree.insert('J', 'J');
        tree.insert('K', 'K');
        tree.insert('M', 'M');
        tree.insert('D', 'D');
        tree.insert('E', 'E');
        tree.insert('N', 'N');
        tree.insert('O', 'O');
        tree.insert('P', 'P');
        tree.insert('U', 'U');
        tree.insert('V', 'V');
        tree.insert('X', 'X');
        tree.insert('Y', 'Y');
        tree.insert('Z', 'Z');
        tree.insert('R', 'R');
        tree.insert('S', 'S');
        tree.insert('T', 'T');
        assert_node!(
            tree.root,
            vec!['G', 'M', 'P', 'X'],
            vec!['G', 'M', 'P', 'X']
        );
        assert_children!(
            tree.root.as_ref().unwrap(),
            [
                vec!['A', 'C', 'D', 'E'],
                vec!['J', 'K'],
                vec!['N', 'O'],
                vec!['R', 'S', 'T', 'U', 'V'],
                vec!['Y', 'Z'],
            ]
        );

        tree.insert('B', 'B');
        assert_node!(
            tree.root,
            vec!['G', 'M', 'P', 'X'],
            vec!['G', 'M', 'P', 'X']
        );
        assert_children!(
            tree.root.as_ref().unwrap(),
            [
                vec!['A', 'B', 'C', 'D', 'E'],
                vec!['J', 'K'],
                vec!['N', 'O'],
                vec!['R', 'S', 'T', 'U', 'V'],
                vec!['Y', 'Z'],
            ]
        );

        tree.insert('Q', 'Q');
        assert_node!(
            tree.root,
            vec!['G', 'M', 'P', 'T', 'X'],
            vec!['G', 'M', 'P', 'T', 'X']
        );
        assert_children!(
            tree.root.as_ref().unwrap(),
            [
                vec!['A', 'B', 'C', 'D', 'E'],
                vec!['J', 'K'],
                vec!['N', 'O'],
                vec!['Q', 'R', 'S'],
                vec!['U', 'V'],
                vec!['Y', 'Z'],
            ]
        );

        tree.insert('L', 'L');
        assert_node!(tree.root, vec!['P'], vec!['P']);
        assert_children!(
            tree.root.as_ref().unwrap(),
            [vec!['G', 'M'], vec!['T', 'X'],]
        );
        assert_children!(
            tree.root
                .as_ref()
                .unwrap()
                .borrow()
                .children
                .as_ref()
                .unwrap()[0],
            [
                vec!['A', 'B', 'C', 'D', 'E'],
                vec!['J', 'K', 'L'],
                vec!['N', 'O']
            ]
        );
        assert_children!(
            tree.root
                .as_ref()
                .unwrap()
                .borrow()
                .children
                .as_ref()
                .unwrap()[1],
            [vec!['Q', 'R', 'S'], vec!['U', 'V'], vec!['Y', 'Z']]
        );

        tree.insert('F', 'F');
        assert_node!(tree.root, vec!['P'], vec!['P']);
        assert_children!(
            tree.root.as_ref().unwrap(),
            [vec!['C', 'G', 'M'], vec!['T', 'X'],]
        );
        assert_children!(
            tree.root
                .as_ref()
                .unwrap()
                .borrow()
                .children
                .as_ref()
                .unwrap()[0],
            [
                vec!['A', 'B'],
                vec!['D', 'E', 'F'],
                vec!['J', 'K', 'L'],
                vec!['N', 'O']
            ]
        );
        assert_children!(
            tree.root
                .as_ref()
                .unwrap()
                .borrow()
                .children
                .as_ref()
                .unwrap()[1],
            [vec!['Q', 'R', 'S'], vec!['U', 'V'], vec!['Y', 'Z']]
        );
    }
}
