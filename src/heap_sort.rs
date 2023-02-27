
pub fn heap_sort<T: Ord>(array: &mut [T]) {
  let n = array.len();
  let mut i = n / 2 - 1;
  let mut j = n - 1;

  while i > 0 {
    heapify(array, i, n);
    i -= 1;
  }

  while j > 0 {
    array.swap(0, j);
    heapify(array, 0, j);
    j -= 1;
  }
}

fn heapify<T: Ord>(array: &mut [T], i: usize, n: usize) {
  let mut largest = i;
  let left = 2 * i + 1;
  let right = 2 * i + 2;

  if left < n && array[left] > array[largest] {
    largest = left;
  }

  if right < n && array[right] > array[largest] {
    largest = right;
  }

  if largest != i {
    array.swap(i, largest);
    heapify(array, largest, n);
  }
}
