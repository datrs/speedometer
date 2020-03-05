#![forbid(unsafe_code, bad_style, future_incompatible)]
#![forbid(rust_2018_idioms, rust_2018_compatibility)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! Measure throughput per second. Adapted from
//! [mafintosh/speedometer](https://github.com/mafintosh/speedometer).
//!
//! ## Why?
//! It's often useful to figure out the average over a sliding timeframe. For
//! example: "how many bytes did we receive on average over the last 5
//! seconds?". Or anything similar. This module allows you to do so in
//! synchronous code.
//!
//! ## WebAssembly
//! When targeting WebAssembly, enable either the `stdweb` feature or the
//! `wasm-bindgen` feature, depending on what you use.
//!
//! ## Examples
//! ```rust
//! extern crate speedometer;
//! use speedometer::Speedometer;
//! use std::time::Duration;
//!
//! let window_size = Duration::from_secs(5); // default is 5 second window size
//! let mut meter = Speedometer::new(window_size);
//! meter.entry(10);
//!
//! println!("{:?} bytes/second!", meter.measure());
//! ```
//!
use instant::Instant;
use std::collections::vec_deque::VecDeque;
use std::time::Duration;

/// Entries into the queue.
#[derive(Debug)]
pub struct Entry {
  timestamp: Instant,
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
      timestamp: Instant::now(),
      value,
    });
  }

  /// Measure the speed.
  pub fn measure(&mut self) -> usize {
    let mut max = 0;
    for (index, entry) in self.queue.iter_mut().enumerate() {
      if entry.timestamp.elapsed() > self.window_size {
        self.total_value -= entry.value;
      } else {
        max = index;
        break;
      }
    }

    for _ in 0..max {
      self.queue.pop_front();
    }

    if self.queue.is_empty() {
      0
    } else {
      self.total_value / self.queue.len()
    }
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
