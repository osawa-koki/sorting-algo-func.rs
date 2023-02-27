
pub fn is_sorted<T: Ord>(array: &[T]) -> bool {
  let n = array.len();
  let mut i = 0;

  while i < n - 1 {
    if array[i] > array[i + 1] {
      return false;
    }

    i += 1;
  }

  true
}
