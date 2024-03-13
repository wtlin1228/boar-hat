/// Counting sort assumes that each of the n input elements is
/// an integer in the range 0 to k. So it can run in Θ(n) time.
/// The space complexity is Θ(n) since counting sort uses two
/// extra vectors with size k and size n respectively.
pub struct CountingSort;

pub trait Id {
    fn id(&self) -> usize;
}

impl Id for usize {
    fn id(&self) -> usize {
        *self
    }
}

impl CountingSort {
    pub fn sort<T: Id + Clone + Default>(&self, slice: &[T], k: usize) -> Vec<T> {
        let mut c: Vec<usize> = Vec::with_capacity(k + 1);
        c.resize(c.capacity(), 0);

        for n in slice {
            c[n.id()] = c[n.id()] + 1;
        }
        for i in 1..=k {
            c[i] = c[i] + c[i - 1];
        }

        let mut res: Vec<T> = Vec::with_capacity(slice.len());
        res.resize(res.capacity(), Default::default());
        for n in slice.iter().rev() {
            res[c[n.id()] - 1] = n.clone();
            c[n.id()] -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_counting_sort() {
        let things = vec![4, 4, 2, 1, 1, 1, 2, 2, 4];
        let sorted = CountingSort.sort(&things, 4);
        assert_eq!(sorted, [1, 1, 1, 2, 2, 2, 4, 4, 4]);
    }

    #[test]
    fn test_large_size() {
        let mut rng = thread_rng();
        let mut things = Vec::with_capacity(5000000);
        let k = 4000000;
        for _ in 0..things.capacity() {
            things.push(rng.gen_range(0..=k));
        }
        let sorted = CountingSort.sort(&things, k);
        for i in 1..sorted.len() {
            assert!(sorted[i] >= sorted[i - 1]);
        }
    }
}
