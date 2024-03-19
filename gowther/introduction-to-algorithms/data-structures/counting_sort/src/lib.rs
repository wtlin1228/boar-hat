pub struct CountingSort;

pub trait SortKey {
    fn key(&self) -> usize;
}

impl SortKey for usize {
    fn key(&self) -> usize {
        *self
    }
}

impl CountingSort {
    // Time and space complexity are both Θ(n + k), where
    //     - n is slice.len()
    //     - k is the maximum key
    pub fn sort<T: SortKey + std::fmt::Debug>(&self, slice: &mut [T]) {
        if slice.len() == 0 {
            return;
        }
        let max = slice.iter().map(|x| x.key()).max().unwrap();
        let mut lookup_table: Vec<usize> = Vec::with_capacity(max + 1);
        lookup_table.resize(max + 1, 0);
        for key in slice.iter().map(|x| x.key()) {
            lookup_table[key] += 1;
        }
        // now lookup table contains the count of each key

        for i in 1..lookup_table.len() {
            lookup_table[i] += lookup_table[i - 1];
        }
        // now lookup table contains the ending index of each key

        // Since Rust doesn't allow us to leave holes inside slices.
        // So I use one additional vector `fixed` to track is this
        // position is already filled with the sorted element.
        // This takes extra Θ(n) space.
        let mut fixed = Vec::with_capacity(slice.len());
        fixed.resize(slice.len(), false);
        let mut ptr = slice.len() - 1;
        while ptr > 0 {
            match fixed[ptr] {
                true => ptr -= 1,
                false => {
                    let move_to = lookup_table[slice[ptr].key()] - 1;
                    lookup_table[slice[ptr].key()] -= 1;
                    slice.swap(move_to, ptr);
                    fixed[move_to] = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_counting_sort() {
        let mut things = vec![4, 4, 2, 1, 1, 1, 2, 2, 4];
        CountingSort.sort(&mut things);
        println!("{:?}", things);
        assert_eq!(things, [1, 1, 1, 2, 2, 2, 4, 4, 4]);
    }

    #[test]
    fn test_large_size() {
        let mut rng = thread_rng();
        let mut things = Vec::with_capacity(5000000);
        let k = 4000000;
        for _ in 0..things.capacity() {
            things.push(rng.gen_range(0..=k));
        }
        CountingSort.sort(&mut things);
        for i in 1..things.len() {
            assert!(things[i] >= things[i - 1]);
        }
    }
}
