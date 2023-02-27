
pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
  let n = arr.len();
  merge_sort_impl(arr, 0, n - 1);
}

fn merge_sort_impl<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) {
  if low < high {
      let mid = (low + high) / 2;
      merge_sort_impl(arr, low, mid);
      merge_sort_impl(arr, mid + 1, high);
      merge(arr, low, mid, high);
  }
}

fn merge<T: Ord + Copy>(arr: &mut [T], low: usize, mid: usize, high: usize) {
  let mut i = low;
  let mut j = mid + 1;
  let mut k = 0;
  let mut temp = Vec::with_capacity(high - low + 1);

  while i <= mid && j <= high {
      if arr[i] < arr[j] {
          temp.push(arr[i]);
          i += 1;
      } else {
          temp.push(arr[j]);
          j += 1;
      }
      k += 1;
  }

  while i <= mid {
      temp.push(arr[i]);
      i += 1;
      k += 1;
  }

  while j <= high {
      temp.push(arr[j]);
      j += 1;
      k += 1;
  }

  for i in 0..k {
      arr[low + i] = temp[i];
  }
}
