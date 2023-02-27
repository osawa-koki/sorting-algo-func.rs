
pub fn insertion_sort<T: Ord>(array: &mut [T]) {
  for i in 1..array.len() {
    let mut j = i;

    while j > 0 && array[j - 1] > array[j] {
      array.swap(j - 1, j);
      j -= 1;
    }
  }
}
