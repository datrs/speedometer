#![feature(external_doc)]
#![doc(include = "../README.md")]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

extern crate failure;

use failure::Error;
use std::collections::vec_deque::VecDeque;
use std::time::{Duration, SystemTime};

/// Entries into the queue.
#[derive(Debug)]
pub struct Entry {
  timestamp: SystemTime,
  value: usize,
}

/// Measure speed in bytes/second.
#[derive(Debug)]
pub struct Speedometer {
  /// Size of the window over which we measure entries.
  pub window_size: Duration,
  queue: VecDeque<Entry>,
  total_value: usize,
}

impl Speedometer {
  /// Create a new instance.
  pub fn new(window_size: Duration) -> Self {
    Self {
      total_value: 0,
      queue: VecDeque::new(),
      window_size,
    }
  }

  /// Create a new instance with a queue of `capacity`.
  pub fn with_capacity(window_size: Duration, capacity: usize) -> Self {
    Self {
      total_value: 0,
      queue: VecDeque::with_capacity(capacity),
      window_size,
    }
  }

  /// Create a new instance with a new queue. Useful if you have prior knowledge
  /// of how big the allocation for the queue should be.
  pub fn with_queue(window_size: Duration, queue: VecDeque<Entry>) -> Self {
    assert!(queue.is_empty());
    Self {
      total_value: 0,
      queue,
      window_size,
    }
  }

  /// Enter a data point into the speedometer.
  pub fn entry(&mut self, value: usize) {
    self.total_value += value;
    self.queue.push_back(Entry {
      timestamp: SystemTime::now(),
      value,
    });
  }

  /// Measure the speed.
  pub fn measure(&mut self) -> Result<usize, Error> {
    let mut max = 0;
    for (index, entry) in self.queue.iter_mut().enumerate() {
      if entry.timestamp.elapsed()? > self.window_size {
        self.total_value -= entry.value;
      } else {
        max = index;
        break;
      }
    }

    for _ in 0..max {
      self.queue.pop_front();
    }

    Ok(self.total_value / self.queue.len())
  }
}

impl Default for Speedometer {
  fn default() -> Self {
    Self {
      window_size: Duration::from_secs(5),
      total_value: 0,
      queue: VecDeque::new(),
    }
  }
}
