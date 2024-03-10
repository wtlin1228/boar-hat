pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubble_sort;
mod insertion_sort;
mod quick_sort;
mod selection_sort;

pub use bubble_sort::BubbleSort;
pub use insertion_sort::InsertionSort;
pub use quick_sort::QuickSort;
pub use selection_sort::SelectionSort;

pub struct StdSorter;
impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

pub struct StdUnstableSorter;
impl<T> Sorter<T> for StdUnstableSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }

    #[test]
    fn stdunstable_works() {
        let mut things = vec![4, 2, 3, 1];
        StdUnstableSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
