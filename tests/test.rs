extern crate speedometer;

use speedometer::Speedometer;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn measures_entries() {
  let window_size = 1;
  let mut meter = Speedometer::new(window_size);
  meter.entry(10);
  meter.entry(10);
  meter.entry(10);
  assert!(
    meter.measure().unwrap() > 0,
    "bytes per second should be non-zero"
  );
  sleep(Duration::from_secs(window_size));
  assert_eq!(meter.measure().unwrap(), 0);
}
