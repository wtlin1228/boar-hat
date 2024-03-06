use std::cmp::Ordering;

trait Heap {
    fn heap_size(&self) -> usize;
    fn exchange(&mut self, i: usize, j: usize);
    fn compare(&self, i: usize, j: usize) -> Ordering;

    #[inline]
    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    #[inline]
    fn left(i: usize) -> usize {
        i * 2 + 1
    }

    #[inline]
    fn right(i: usize) -> usize {
        i * 2 + 2
    }
}

trait MaxHeap: Heap {
    fn max_heapify_up(&mut self, current: usize) {
        if current == 0 {
            return;
        }
        let parent = Self::parent(current);
        match self.compare(current, parent) {
            Ordering::Greater => {
                self.exchange(current, parent);
                self.max_heapify_up(parent);
            }
            _ => (),
        }
    }

    fn max_heapify_down(&mut self, i: usize, boundary: Option<usize>) {
        let l = Self::left(i);
        let r = Self::right(i);
        let mut heap_size = self.heap_size();
        if let Some(boundary) = boundary {
            if boundary < heap_size {
                heap_size = boundary;
            }
        }
        let mut largest = i;
        if l < heap_size && self.compare(l, largest) == Ordering::Greater {
            largest = l;
        }
        if r < heap_size && self.compare(r, largest) == Ordering::Greater {
            largest = r;
        }
        if largest != i {
            self.exchange(i, largest);
            self.max_heapify_down(largest, boundary);
        }
    }

    fn build_max_heap(&mut self) {
        let mut i = Self::parent(self.heap_size() - 1) as i32;
        while i >= 0 {
            self.max_heapify_down(i as usize, None);
            i -= 1;
        }
    }
}

trait MinHeap: Heap {
    fn min_heapify_up(&mut self, current: usize) {
        if current == 0 {
            return;
        }
        let parent = Self::parent(current);
        match self.compare(current, parent) {
            Ordering::Less => {
                self.exchange(current, parent);
                self.min_heapify_up(parent);
            }
            _ => (),
        }
    }

    fn min_heapify_down(&mut self, i: usize, boundary: Option<usize>) {
        let l = Self::left(i);
        let r = Self::right(i);
        let mut heap_size = self.heap_size();
        if let Some(boundary) = boundary {
            if boundary < heap_size {
                heap_size = boundary;
            }
        }
        let mut largest = i;
        if l < heap_size && self.compare(l, largest) == Ordering::Less {
            largest = l;
        }
        if r < heap_size && self.compare(r, largest) == Ordering::Less {
            largest = r;
        }
        if largest != i {
            self.exchange(i, largest);
            self.min_heapify_down(largest, boundary);
        }
    }

    fn build_min_heap(&mut self) {
        let mut i = Self::parent(self.heap_size() - 1) as i32;
        while i >= 0 {
            self.min_heapify_down(i as usize, None);
            i -= 1;
        }
    }
}

trait HeapSort: MaxHeap {
    fn heap_sort(&mut self) {
        self.build_max_heap();
        let mut i = self.heap_size() - 1;
        loop {
            self.exchange(0, i);
            self.max_heapify_down(0, Some(i));
            match i {
                1 => break,
                _ => i -= 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    impl<T: Ord> Heap for Vec<T> {
        fn heap_size(&self) -> usize {
            self.len()
        }

        fn exchange(&mut self, i: usize, j: usize) {
            self.swap(i, j);
        }

        fn compare(&self, i: usize, j: usize) -> Ordering {
            self[i].cmp(&self[j])
        }
    }

    impl<T: Ord> Heap for &mut [T] {
        fn heap_size(&self) -> usize {
            self.len()
        }

        fn exchange(&mut self, i: usize, j: usize) {
            self.swap(i, j);
        }

        fn compare(&self, i: usize, j: usize) -> Ordering {
            self[i].cmp(&self[j])
        }
    }

    impl<T: Ord> MaxHeap for Vec<T> {}
    impl<T: Ord> MaxHeap for &mut [T] {}

    impl<T: Ord> MinHeap for Vec<T> {}
    impl<T: Ord> MinHeap for &mut [T] {}

    impl<T: Ord> HeapSort for Vec<T> {}
    impl<T: Ord> HeapSort for &mut [T] {}

    mod max_heap_tests {
        use super::MaxHeap;

        #[test]
        fn test_vector() {
            let mut v = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
            v.build_max_heap();
            assert_eq!(v, [16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
        }

        #[test]
        fn test_slice() {
            let mut v = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7, 5, 6];
            let mut slice = &mut v[..10];
            slice.build_max_heap();
            assert_eq!(v, [16, 14, 10, 8, 7, 9, 3, 2, 4, 1, 5, 6]);
        }

        #[test]
        fn test_array() {
            let mut v = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7, 5, 6];
            let mut slice = &mut v[..10];
            slice.build_max_heap();
            assert_eq!(v, [16, 14, 10, 8, 7, 9, 3, 2, 4, 1, 5, 6]);
        }
    }

    mod min_heap_tests {
        use super::MinHeap;

        #[test]
        fn test_vector() {
            let mut v = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7, 5, 6];
            v.build_min_heap();
            assert_eq!(v, [1, 2, 3, 4, 5, 6, 10, 14, 8, 7, 16, 9]);
        }

        #[test]
        fn test_slice() {
            let mut v = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7, 5, 6];
            let mut slice = &mut v[..10];
            slice.build_min_heap();
            assert_eq!(v, [1, 2, 3, 4, 7, 9, 10, 14, 8, 16, 5, 6]);
        }

        #[test]
        fn test_array() {
            let mut v = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7, 5, 6];
            let mut slice = &mut v[..10];
            slice.build_min_heap();
            assert_eq!(v, [1, 2, 3, 4, 7, 9, 10, 14, 8, 16, 5, 6]);
        }
    }

    mod heap_sort_tests {
        use super::HeapSort;

        #[test]
        fn test_vector() {
            let mut v = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
            v.heap_sort();
            assert_eq!(v, [1, 2, 3, 4, 7, 8, 9, 10, 14, 16]);
        }

        #[test]
        fn test_slice() {
            let mut v = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7, 5, 6];
            let mut slice = &mut v[..10];
            slice.heap_sort();
            assert_eq!(v, [1, 2, 3, 4, 7, 8, 9, 10, 14, 16, 5, 6]);
        }

        #[test]
        fn test_array() {
            let mut v = [4, 1, 3, 2, 16, 9, 10, 14, 8, 7, 5, 6];
            let mut slice = &mut v[..10];
            slice.heap_sort();
            assert_eq!(v, [1, 2, 3, 4, 7, 8, 9, 10, 14, 16, 5, 6]);
        }
    }
}
