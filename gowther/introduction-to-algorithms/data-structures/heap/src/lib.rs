use std::cmp::Ordering;

trait Heapify {
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

trait MaxHeapify: Heapify {
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

trait MinHeapify: Heapify {
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

trait HeapSort: MaxHeapify {
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

impl<T: Ord> Heapify for Vec<T> {
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

impl<T: Ord> Heapify for &mut [T] {
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

impl<T: Ord> MaxHeapify for Vec<T> {}
impl<T: Ord> MaxHeapify for &mut [T] {}

impl<T: Ord> MinHeapify for Vec<T> {}
impl<T: Ord> MinHeapify for &mut [T] {}

impl<T: Ord> HeapSort for Vec<T> {}
impl<T: Ord> HeapSort for &mut [T] {}

pub struct MaxHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from(mut data: Vec<T>) -> Self {
        data.build_max_heap();
        Self { data }
    }

    pub fn insert(&mut self, v: T) {
        self.data.push(v);
        self.data.max_heapify_up(self.data.len() - 1);
    }

    pub fn delete(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            return None;
        }
        self.data.exchange(0, self.data.len() - 1);
        let res = self.data.pop().unwrap();
        self.data.max_heapify_down(0, None);
        Some(res)
    }

    pub fn peek(&self) -> Option<&T> {
        match self.data.len() {
            0 => None,
            _ => Some(&self.data[0]),
        }
    }
}

pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from(mut data: Vec<T>) -> Self {
        data.build_min_heap();
        Self { data }
    }

    pub fn insert(&mut self, v: T) {
        self.data.push(v);
        self.data.min_heapify_up(self.data.len() - 1);
    }

    pub fn delete(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            return None;
        }
        self.data.exchange(0, self.data.len() - 1);
        let res = self.data.pop().unwrap();
        self.data.min_heapify_down(0, None);
        Some(res)
    }

    pub fn peek(&self) -> Option<&T> {
        match self.data.len() {
            0 => None,
            _ => Some(&self.data[0]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod max_heapify_tests {
        use super::MaxHeapify;

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

    mod min_heapify_tests {
        use super::MinHeapify;

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

    mod max_heap_tests {
        use super::MaxHeap;

        #[test]
        fn test_new() {
            let mut q: MaxHeap<i32> = MaxHeap::new();
            for i in [4, 1, 3, 2, 16, 9, 10, 14, 8, 7] {
                q.insert(i);
            }

            for i in [16, 14, 10, 9, 8, 7, 4, 3, 2, 1] {
                assert_eq!(q.peek().unwrap(), &i);
                assert_eq!(q.delete().unwrap(), i);
            }
        }

        #[test]
        fn test_from() {
            let mut q = MaxHeap::from(vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7]);
            for i in [16, 14, 10, 9, 8, 7, 4, 3, 2, 1] {
                assert_eq!(q.peek().unwrap(), &i);
                assert_eq!(q.delete().unwrap(), i);
            }
        }
    }

    mod min_heap_tests {
        use super::MinHeap;

        #[test]
        fn test_new() {
            let mut q: MinHeap<i32> = MinHeap::new();
            for i in [4, 1, 3, 2, 16, 9, 10, 14, 8, 7] {
                q.insert(i);
            }

            for i in [1, 2, 3, 4, 7, 8, 9, 10, 14, 16] {
                assert_eq!(q.peek().unwrap(), &i);
                assert_eq!(q.delete().unwrap(), i);
            }
        }

        #[test]
        fn test_from() {
            let mut q = MinHeap::from(vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7]);
            for i in [1, 2, 3, 4, 7, 8, 9, 10, 14, 16] {
                assert_eq!(q.peek().unwrap(), &i);
                assert_eq!(q.delete().unwrap(), i);
            }
        }
    }
}
