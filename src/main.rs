mod display;
mod shuffle;

use display::display;
use shuffle::shuffle;

fn main() {
  let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

  display("Original", &array);

  shuffle(&mut array);
  display("Shuffled", &array);
}
