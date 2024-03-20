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
    pub fn sort<T: SortKey>(&self, slice: &mut [T]) {
        if slice.len() == 0 {
            return;
        }
        let max = slice.iter().map(|x| x.key()).max().unwrap();
        let mut lookup_table: Vec<usize> = Vec::with_capacity(max + 1);
        lookup_table.resize(max + 1, 0);
        for key in slice.iter().map(|x| x.key()) {
            lookup_table[key] += 1;
        }
        // now the lookup table contains the count of each key

        for i in 1..lookup_table.len() {
            lookup_table[i] += lookup_table[i - 1];
        }
        // now the lookup table contains the ending index of each key

        // Since Rust doesn't allow us to leave holes inside slices.
        // Use a index table to trace ending position for each element.
        // This takes extra Θ(n) space.
        let mut index_table = Vec::with_capacity(slice.len());
        index_table.resize(slice.len(), 0);
        slice.iter().enumerate().rev().for_each(|(idx, x)| {
            index_table[idx] = lookup_table[x.key()] - 1;
            lookup_table[x.key()] -= 1;
        });
        // now the index table contains the position where each element should
        // be ending at.

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
    fn test_is_stable() {
        struct Foo {
            name: String,
            key: usize,
        }

        impl SortKey for Foo {
            fn key(&self) -> usize {
                self.key
            }
        }

        let mut things = vec![
            Foo {
                name: "Kirby".to_string(),
                key: 1,
            },
            Foo {
                name: "PUI PUI".to_string(),
                key: 4,
            },
            Foo {
                name: "Pikachu".to_string(),
                key: 3,
            },
            Foo {
                name: "Leo".to_string(),
                key: 2,
            },
            Foo {
                name: "Una".to_string(),
                key: 0,
            },
            Foo {
                name: "Pichu".to_string(),
                key: 1,
            },
            Foo {
                name: "Hawk".to_string(),
                key: 3,
            },
            Foo {
                name: "BeruBō".to_string(),
                key: 2,
            },
            Foo {
                name: "Happy".to_string(),
                key: 6,
            },
            Foo {
                name: "Marill".to_string(),
                key: 9,
            },
        ];
        CountingSort.sort(&mut things);
        assert_eq!(
            things.iter().map(|x| &x.name[..]).collect::<Vec<&str>>(),
            [
                "Una", "Kirby", "Pichu", "Leo", "BeruBō", "Pikachu", "Hawk", "PUI PUI", "Happy",
                "Marill",
            ]
        );
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
