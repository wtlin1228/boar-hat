pub mod avl_tree;
pub mod red_black_tree;

pub trait Key<T: Ord> {
    fn key(&self) -> &T;
}
