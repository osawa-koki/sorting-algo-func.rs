mod display;
mod shuffle;

use display::display;
use shuffle::shuffle;

fn main() {
  let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  display("Original", &array);
  shuffle(&mut array.to_vec());
  display("Shuffled", &array);
}
