
pub fn bubble_sort<T: Ord>(array: &mut [T]) {
  let mut swapped = true;
  let mut last = array.len() - 1;

  while swapped {
    swapped = false;

    for i in 0..last {
      if array[i] > array[i + 1] {
        array.swap(i, i + 1);
        swapped = true;
      }
    }

    last -= 1;
  }
}
