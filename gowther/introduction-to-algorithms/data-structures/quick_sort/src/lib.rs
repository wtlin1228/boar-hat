use rand::prelude::*;

pub struct QuickSort {
    randomized: bool,
}

impl QuickSort {
    /// Worst case for quick sort is Θ(n^2) which is only
    /// happened when the slice is already sorted. However,
    /// that's rare and the expected time complexity is
    /// Θ(n log n). We can leverage on the random partition
    /// trick for more confidence.
    pub fn sort<T: Ord>(&self, slice: &mut [T]) {
        match slice.len() {
            0 | 1 => return,
            2 => {
                if slice[0] > slice[1] {
                    slice.swap(0, 1);
                }
                return;
            }
            _ => (),
        }
        let p = self.partition(slice);
        self.sort(&mut slice[0..p]);
        self.sort(&mut slice[p + 1..]);
    }

    fn partition<T: Ord>(&self, slice: &mut [T]) -> usize {
        let last_idx = slice.len() - 1;
        if self.randomized {
            let mut rng = thread_rng();
            let i = rng.gen_range(0..=last_idx);
            slice.swap(i, last_idx);
        }
        let (pivot, rest) = slice.split_last_mut().expect("slice is non-empty");
        let mut l = 0;
        let mut r = rest.len() - 1;
        while l <= r {
            if &rest[l] <= pivot {
                l += 1;
            } else if &rest[r] > pivot {
                if r == 0 {
                    break;
                }
                r -= 1;
            } else {
                rest.swap(l, r);
                l += 1;
                if r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        slice.swap(l, last_idx);
        return l;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut things = vec![4, 2, 5, 3, 1];
        QuickSort { randomized: false }.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_partition() {
        let mut things = vec![4, 2, 5, 3, 1];
        QuickSort { randomized: true }.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_large_size() {
        let mut rand = rand::thread_rng();
        let mut things = Vec::with_capacity(5000000);
        for _ in 0..things.capacity() {
            things.push(rand.gen::<usize>());
        }
        QuickSort { randomized: false }.sort(&mut things);
        for i in 1..things.len() {
            assert!(things[i] >= things[i - 1]);
        }
    }

    #[test]
    fn test_large_size_with_random_partition() {
        let mut rand = rand::thread_rng();
        let mut things = Vec::with_capacity(5000000);
        for _ in 0..things.capacity() {
            things.push(rand.gen::<usize>());
        }
        QuickSort { randomized: true }.sort(&mut things);
        for i in 1..things.len() {
            assert!(things[i] >= things[i - 1]);
        }
    }
}
