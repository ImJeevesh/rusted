pub fn fibonacci(num: u64) -> u64 {
  match num {
    0 | 1 | 2 => 1,
    _ => fibonacci(num - 1) + fibonacci(num - 2),
  }
}
