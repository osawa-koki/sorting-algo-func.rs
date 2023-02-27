mod display;
mod shuffle;

mod bubble_sort;

use display::display;
use shuffle::shuffle;

use bubble_sort::bubble_sort;

fn main() {
  let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

  display("Original", &array);

  shuffle(&mut array);
  display("Shuffled", &array);

  bubble_sort(&mut array);
  display("Bubble Sort", &array);
}
