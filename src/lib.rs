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
  window_size: u64,
  queue: VecDeque<Entry>,
  total_value: usize,
}

impl Speedometer {
  /// Create a new instance.
  pub fn new(window_size: u64) -> Self {
    Self {
      total_value: 0,
      queue: VecDeque::new(),
      window_size,
    }
  }

  /// Create a new instance with a new queue. Useful if you have prior knowledge
  /// of how big the allocation for the queue should be.
  pub fn with_queue(window_size: u64, queue: VecDeque<Entry>) -> Self {
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
    let expiry = Duration::from_secs(self.window_size);

    let mut max = 0;
    for (index, entry) in self.queue.iter_mut().enumerate() {
      if entry.timestamp.elapsed()? > expiry {
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
      window_size: 5,
      total_value: 0,
      queue: VecDeque::new(),
    }
  }
}
