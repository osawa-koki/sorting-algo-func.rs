mod display;
mod shuffle;

mod bubble_sort;
mod insertion_sort;
mod selection_sort;
mod merge_sort;

use display::display;
use shuffle::shuffle;

use bubble_sort::bubble_sort;
use insertion_sort::insertion_sort;
use selection_sort::selection_sort;
use merge_sort::merge_sort;

fn main() {
  let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

  display("Original", &array);

  shuffle(&mut array);
  display("Shuffled", &array);

  bubble_sort(&mut array);
  display("Bubble Sort", &array);

  shuffle(&mut array);
  display("Shuffled", &array);

  insertion_sort(&mut array);
  display("Insertion Sort", &array);

  shuffle(&mut array);
  display("Shuffled", &array);

  selection_sort(&mut array);
  display("Selection Sort", &array);

  shuffle(&mut array);
  display("Shuffled", &array);

  merge_sort(&mut array);
  display("Merge Sort", &array);
}
