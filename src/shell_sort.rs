
pub fn shell_sort<T: Ord>(array: &mut [T]) {
  let n = array.len();
  let mut gap = n / 2;

  while gap > 0 {
    for i in gap..n {
      let mut j = i;

      while j >= gap && array[j - gap] > array[j] {
        array.swap(j - gap, j);
        j -= gap;
      }
    }

    gap /= 2;
  }
}
