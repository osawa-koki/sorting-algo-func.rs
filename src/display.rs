
pub fn display(s: &str, arr: &[i32]) {
  let mut nums = String::new();
  for num in arr {
    nums.push_str(&num.to_string());
    nums.push(' ');
  }
  nums.pop();
  println!("{:>20}: {}", s, nums);
}
