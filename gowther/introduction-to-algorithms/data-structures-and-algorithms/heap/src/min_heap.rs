use std::cmp::Ordering;

pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn from(mut data: Vec<T>) -> Self {
        Self::build_min_heap(&mut data[..]);
        Self { data }
    }

    pub fn insert(&mut self, v: T) {
        self.data.push(v);
        let last_idx = self.data.len() - 1;
        Self::min_heapify_up(&mut self.data[..], last_idx);
    }

    pub fn delete(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            return None;
        }
        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);
        let res = self.data.pop().unwrap();
        Self::min_heapify_down(&mut self.data[..], 0);
        Some(res)
    }

    pub fn peek(&self) -> Option<&T> {
        match self.data.len() {
            0 => None,
            _ => Some(&self.data[0]),
        }
    }

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

    fn build_min_heap(slice: &mut [T]) {
        for i in (0..=Self::parent(slice.len() - 1)).rev() {
            Self::min_heapify_down(slice, i);
        }
    }

    fn min_heapify_down(slice: &mut [T], i: usize) {
        let l = Self::left(i);
        let r = Self::right(i);
        let mut largest = i;
        if l < slice.len() && slice[l].cmp(&slice[largest]) == Ordering::Less {
            largest = l;
        }
        if r < slice.len() && slice[r].cmp(&slice[largest]) == Ordering::Less {
            largest = r;
        }
        if largest != i {
            slice.swap(i, largest);
            Self::min_heapify_down(slice, largest);
        }
    }

    fn min_heapify_up(slice: &mut [T], current: usize) {
        if current == 0 {
            return;
        }
        let parent = Self::parent(current);
        match slice[current].cmp(&slice[parent]) {
            Ordering::Less => {
                slice.swap(current, parent);
                Self::min_heapify_up(slice, parent);
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_min_heap() {
        let mut v = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        MinHeap::build_min_heap(&mut v[..]);
        assert_eq!(v, [1, 2, 3, 4, 7, 9, 10, 14, 8, 16]);
    }

    #[test]
    fn test_new_min_heap() {
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
    fn test_from_vec() {
        let mut q = MinHeap::from(vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7]);
        for i in [1, 2, 3, 4, 7, 8, 9, 10, 14, 16] {
            assert_eq!(q.peek().unwrap(), &i);
            assert_eq!(q.delete().unwrap(), i);
        }
    }
}
