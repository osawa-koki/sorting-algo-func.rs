
pub fn shuffle<T: Copy>(array: &mut [T]) {
  for i in 0..array.len() {
    let j = rand::random::<usize>() % (array.len() - i) + i;
    array.swap(i, j);
  }
}
