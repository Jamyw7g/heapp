# heapp
Some heap operations on slice



## Example

The following examples show a quick example of some of the very basic functionality of `heapp`.

```Rust
use heapp::*;

fn main() {
  let mut heap = vec![4, 7, 8, 3, 4, 90, 78, 67, 90];
  heapify(&mut heap);	// build a heap
  heap.push(32);
  heap_push(&mut heap); // push an element into the heap
  heap_pop(&mut heap); // pop an element from the heap
  let ele = heap.pop().unwrap();
  
  heap_sort(&mut heap); // same as heap.sort(), but using heap sort algorithm
  
  let res = n_smallest(&mut heap, 3); // res == [3, 4, 4]
}
```

