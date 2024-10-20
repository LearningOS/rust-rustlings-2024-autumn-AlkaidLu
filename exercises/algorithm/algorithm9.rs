/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        if self.count >= self.items.len() {
            self.items.push(T::default()); // 确保容量足够
        }
        self.items[self.count] = value; // 放置新元素
        self.bubble_up(self.count); // 上浮操作
    }

    fn bubble_up(&mut self, idx: usize) {
        let mut current_idx = idx;
        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx);
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn sink_down(&mut self, idx: usize) {
        let mut current_idx = idx;
        
        while self.children_present(current_idx) {
            let smallest_child_idx = self.smallest_child_idx(current_idx);
            
            // 比较当前节点和最小子节点，如果当前节点大于子节点，则交换
            if (self.comparator)(&self.items[smallest_child_idx], &self.items[current_idx]) {
                self.items.swap(current_idx, smallest_child_idx);
                current_idx = smallest_child_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right // 如果右子节点优先级更高，则返回右子节点
        } else {
            left // 否则返回左子节点
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count==0{
            return None;
        }
		// 获取堆顶（根）的值
        let root_value = std::mem::replace(&mut self.items[1], T::default()); // 索引 1 是根节点，取出值

        // 将最后一个元素移动到堆顶位置，并减少堆大小
        self.items[1] = std::mem::replace(&mut self.items[self.count], T::default());
        self.count -= 1;

        // 对根进行下沉操作，维护堆的性质
        if self.count > 0 {
            self.sink_down(1);
        }

        // 返回原根元素
        Some(root_value)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}