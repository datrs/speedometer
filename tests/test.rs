extern crate speedometer;

use speedometer::Speedometer;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn measures_entries() {
  let window_size = Duration::from_secs(1);
  let mut meter = Speedometer::new(window_size);
  meter.entry(10);
  meter.entry(10);
  meter.entry(10);
  assert!(meter.measure() > 0, "bytes per second should be non-zero");
  sleep(window_size);
  assert_eq!(meter.measure(), 0);
}

#[test]
fn no_entries() {
  let window_size = Duration::from_secs(1);
  let mut meter = Speedometer::new(window_size);
  assert_eq!(meter.measure(), 0, "should not crash on empty queue");
}
