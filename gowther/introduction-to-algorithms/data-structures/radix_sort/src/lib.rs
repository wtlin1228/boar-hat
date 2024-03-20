use counting_sort::{CountingSort, SortKey};

pub struct RadixSort;

impl RadixSort {
    // Time complexity is Θ(nc) and Space complexity is Θ(n), where
    //     - n is slice.len()
    //     - c is the pow of the largest key based on n, u <= n.pow(c)
    pub fn sort<T: SortKey>(&self, slice: &mut [T]) {
        if slice.len() == 0 {
            return;
        }
        let n = slice.len();
        let u = slice.iter().map(|x| x.key()).max().unwrap();
        let c = get_bit_length(u).div_ceil(get_bit_length(n));

        // calculate the digits for each element
        let mut inners: Vec<Inner> = slice
            .iter()
            .enumerate()
            .map(|(idx, x)| {
                let mut inner = Inner::new(idx, c as usize);
                let mut v = x.key();
                for i in (0..c).rev() {
                    let base = n.pow(i);
                    inner.digits.push(v / base);
                    v = v % base;
                }
                inner
            })
            .collect();

        // counting sort each digit, starting from the least important digit
        for i in (0..c).rev() {
            inners
                .iter_mut()
                .for_each(|inner| inner.key = inner.digits[i as usize]);
            CountingSort.sort(&mut inners);
        }

        // The easy way to sort the slice is creating a new vector, coping each
        // elements in the original slice to the corresponding positions then
        // swap the new created vector with the original slice.
        // But that approach required either T is Copy or Default because Rust
        // doesn't allow us to leave holes inside slice.
        // So, here I sort the slice by more swaps based on the final position
        // which each element should be at. That's somewhat like cyclic sort.
        let mut index_table = Vec::with_capacity(n);
        index_table.resize(n, 0);
        inners.iter().enumerate().for_each(|(idx, inner)| {
            index_table[inner.original_index] = idx;
        });

        // do the sorting
        let mut i = 0;
        while i < index_table.len() {
            match index_table[i] == i {
                true => i += 1,
                false => {
                    let move_to = index_table[i];
                    slice.swap(move_to, i);
                    index_table.swap(move_to, i);
                }
            }
        }
    }
}

fn get_bit_length(x: usize) -> u32 {
    usize::BITS - x.leading_zeros()
}

#[derive(Debug)]
struct Inner {
    original_index: usize,
    key: usize,
    digits: Vec<usize>,
}

impl Inner {
    fn new(original_index: usize, c: usize) -> Self {
        Self {
            original_index,
            key: 0,
            digits: Vec::with_capacity(c),
        }
    }
}

impl SortKey for Inner {
    fn key(&self) -> usize {
        self.key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radix_sort_max_int_is_n() {
        let mut things = vec![1, 4, 3, 2, 0, 1, 3, 2, 6, 8];
        RadixSort.sort(&mut things);
        assert_eq!(things, [0, 1, 1, 2, 2, 3, 3, 4, 6, 8]);
    }

    #[test]
    fn radix_sort_max_int_is_2n() {
        let mut things = vec![1, 14, 33, 42, 40, 41, 13, 42, 16, 99];
        RadixSort.sort(&mut things);
        assert_eq!(things, [1, 13, 14, 16, 33, 40, 41, 42, 42, 99]);
    }

    #[test]
    fn radix_sort_max_int_is_3n() {
        let mut things = vec![111, 124, 31, 142, 540, 741, 513, 432, 156, 24];
        RadixSort.sort(&mut things);
        assert_eq!(things, [24, 31, 111, 124, 142, 156, 432, 513, 540, 741]);
    }
}
