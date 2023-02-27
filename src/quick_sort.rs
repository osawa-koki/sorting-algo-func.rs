
pub fn quick_sort<T: Ord + Copy>(arr: &mut [T]) {
  let n = arr.len();
  quick_sort_impl(arr, 0, n - 1);
}

fn quick_sort_impl<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) {
  if low < high {
    let pivot = partition(arr, low, high);
    quick_sort_impl(arr, low, pivot - 1);
    quick_sort_impl(arr, pivot + 1, high);
  }
}

fn partition<T: Ord + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
  let pivot = arr[high];
  let mut i = low;

  for j in low..high {
    if arr[j] < pivot {
      arr.swap(i, j);
      i += 1;
    }
  }

  arr.swap(i, high);
  i
}
