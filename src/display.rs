
pub fn display(s: &str, arr: &[i32]) {
  let mut nums = String::new();
  let is_sorted = super::is_sorted::is_sorted(arr);
  for num in arr {
    nums.push_str(&num.to_string());
    nums.push(' ');
  }
  nums.pop();
  println!("{:>20} ({:<5}): {}", s, is_sorted, nums);
}
