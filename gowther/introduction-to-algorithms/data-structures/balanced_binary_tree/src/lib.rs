pub mod avl_tree;
pub mod red_black_tree;

pub trait Key<T: Ord> {
    fn key(&self) -> &T;
}

trait DynamicSet<K, V>
where
    K: Ord,
    V: Key<K>,
{
    fn search(k: &K) -> Option<&V>;
    fn predecessor(k: &K) -> Option<&V>;
    fn successor(k: &K) -> Option<&V>;
    fn minimum(k: &K) -> &V;
    fn maximum(k: &K) -> &V;
    fn insert(v: V);
    fn delete(k: &K) -> Result<V, ()>;
}
