//! Some heap operations on slice.

#![no_std]
use core::cmp::Ordering;


#[inline]
fn siftdown_by<T, F>(heap: &mut [T], start: usize, mut pos: usize, mut f: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    while start < pos {
        let p = (pos - 1) >> 1;
        let cmp = unsafe { f(heap.get_unchecked(pos), heap.get_unchecked(p)) };
        if cmp == Ordering::Less {
            heap.swap(p, pos);
            pos = p;
            continue;
        }
        break;
    }
}

#[inline]
fn siftup_by<T, F>(heap: &mut [T], end: usize, mut pos: usize, mut f: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut child = (pos << 1) + 1;
    while child < end {
        let right = child + 1;
        if right < end {
            let cmp = unsafe { f(heap.get_unchecked(right), heap.get_unchecked(child)) };
            if cmp == Ordering::Less {
                child = right;
            }
        }
        let cmp = unsafe { f(heap.get_unchecked(child), heap.get_unchecked(pos)) };
        if cmp == Ordering::Less {
            heap.swap(child, pos);
            pos = child;
            child = (pos << 1) + 1;
            continue;
        }
        break;
    }
}

/// Gives a heap slice, which contains a new element in the tail, using `heap_push` function to
/// extend the heap to include the tail element. Default heap is smallest heap.
#[inline]
pub fn heap_push<T: Ord>(heap: &mut [T]) {
    heap_push_by(heap, |a, b| a.cmp(b));
}

/// Just like `heap_push`, but with a compare function to indicate the order of the heap.
#[inline]
pub fn heap_push_by<T, F>(heap: &mut [T], f: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    siftdown_by(heap, 0, heap.len() - 1, f);
}

/// Move the smallest element to the tail of the heap.
#[inline]
pub fn heap_pop<T: Ord>(heap: &mut [T]) {
    heap_pop_by(heap, |a, b| a.cmp(b))
}

/// Just like `heap_pop`, but with a compare function.
#[inline]
pub fn heap_pop_by<T, F>(heap: &mut [T], f: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = heap.len();
    heap.swap(0, len - 1);
    siftup_by(heap, len - 1, 0, f);
}

/// Get the `n` smallest element from a slice, when n > len, return None.
#[inline]
pub fn n_smallest<T: Ord>(heap: &mut [T], n: usize) -> Option<&[T]> {
    let len = heap.len();
    if len < n {
        None
    } else {
        (0..n).for_each(|i| heapify(&mut heap[i..]));
        Some(&heap[..n])
    }
}

/// To construct a heap from a slice; default heap is smallest heap.
#[inline]
pub fn heapify<T: Ord>(heap: &mut [T]) {
    let len = heap.len();
    (0..len >> 1)
        .rev()
        .for_each(|i| siftup_by(heap, len, i, |a, b| a.cmp(b)));
}

/// Using heap sort algorithm
#[inline]
pub fn heap_sort<T: Ord>(heap: &mut [T]) {
    (0..heap.len()).for_each(|i| heapify(&mut heap[i..]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_test() {
        let mut heap = [4, 6, 7, 8, 3, 5, 9];
        heapify(&mut heap);
        let len = heap.len();

        heap_pop(&mut heap[..]);
        assert_eq!(heap[len-1], 3);
        heap_pop(&mut heap[..len-1]);
        assert_eq!(heap[len-2], 4);

        if let Some(res) = n_smallest(&mut heap, 3) {
            assert_eq!(res, [3, 4, 5]);
        }

        let mut heap = [3, 4, 5, 9, 10, 6, 7, 2];
        heap_push(&mut heap);
        assert_eq!(heap, [2, 3, 5, 4, 10, 6, 7, 9]);

        heap_sort(&mut heap);
        assert_eq!(heap, [2, 3, 4, 5, 6, 7, 9, 10]);
    }
}
