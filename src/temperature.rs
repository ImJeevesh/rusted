fn round(value: f64, precision: u64) -> f64 {
  let b = 10.0 * precision as f64;
  return (value * b).round() / b;
}

pub fn fahrenheit_to_celcius(f: f64) -> f64 {
  return round((f - 32.0) * 5.0 / 9.0, 4);
}

pub fn celcius_to_fahrenheit(c: f64) -> f64 {
  return round((c * 9.0 / 5.0) + 32.0, 4);
}
