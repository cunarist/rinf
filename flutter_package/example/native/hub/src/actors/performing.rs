//! This module is written for Rinf demonstrations.

use crate::signals::{SampleFractal, SampleSchema};
use anyhow::Result;
use async_trait::async_trait;
use messages::prelude::{Actor, Address, Context, Notifiable};
use rinf::{RustSignalBinary, debug_print};
use sample_crate::{
  ImageInfo, draw_fractal_image, fetch_from_web_api, get_current_time,
  get_hardward_id,
};
use std::collections::VecDeque;
use std::time::Duration;
use tokio::spawn;
use tokio::task::{JoinHandle, JoinSet, spawn_blocking, yield_now};
use tokio::time::sleep;
use tokio_with_wasm::alias as tokio;

/// The actor that performs complex calculations.
pub struct PerformingActor {
  _owned_tasks: JoinSet<()>,
}

impl Actor for PerformingActor {}

impl PerformingActor {
  pub fn new(self_addr: Address<Self>) -> Self {
    let mut owned_tasks = JoinSet::new();
    owned_tasks.spawn(Self::run_debug_tests());
    owned_tasks.spawn(Self::stream_fractal(self_addr));
    PerformingActor {
      _owned_tasks: owned_tasks,
    }
  }
}

#[async_trait]
impl Notifiable<ImageInfo> for PerformingActor {
  async fn notify(&mut self, msg: ImageInfo, _: &Context<Self>) {
    // Send the image data to Dart.
    SampleFractal {
      current_scale: msg.scale,
      dummy: Some(SampleSchema {
        sample_field_one: true,
        sample_field_two: false,
      }),
    }
    .send_signal_to_dart(msg.data);
  }
}

impl PerformingActor {
  // Using the `cfg` macro enables conditional statement.
  #[cfg(debug_assertions)]
  const IS_DEBUG_MODE: bool = true;
  #[cfg(not(debug_assertions))]
  const IS_DEBUG_MODE: bool = false;

  /// Continuously draws fractal animation frames.
  async fn stream_fractal(mut self_addr: Address<Self>) {
    // Create a deque to preserve the frame order.
    let max_handles = 16;
    let mut deque = VecDeque::<JoinHandle<Option<ImageInfo>>>::new();

    // Continuously push join handles into the deque
    // that will return images upon completion.
    let mut draw_scale: f64 = 1.0;
    loop {
      // Wait until the next frame's time.
      sleep(Duration::from_millis(40)).await;

      // Check if the deque is full and await the oldest item if it is.
      if deque.len() == max_handles {
        if let Some(join_handle) = deque.pop_front() {
          let image_info = match join_handle.await {
            Ok(Some(inner)) => inner,
            _ => continue,
          };
          let _ = self_addr.notify(image_info).await;
        };
      }

      // Update the current scale.
      draw_scale *= 1.02;
      if draw_scale > 1e+7 {
        draw_scale = 1.0
      };

      // Add the join handle to the deque.
      deque.push_back(spawn(Self::draw_fractal(draw_scale)));
    }
  }

  /// Draws a single fractal animation frame.
  async fn draw_fractal(draw_scale: f64) -> Option<ImageInfo> {
    // Calculate the fractal image
    // parallelly in a separate thread pool.
    let handle = spawn_blocking(move || draw_fractal_image(draw_scale));
    let image_info = match handle.await {
      Ok(Ok(inner)) => inner,
      _ => None?,
    };
    Some(image_info)
  }

  /// A function for testing various capabilities.
  async fn run_debug_tests() {
    if !Self::IS_DEBUG_MODE {
      return;
    }
    match Self::call_debug_functions().await {
      Ok(_) => debug_print!("Debug tests completed!"),
      Err(err) => debug_print!("Debug test error: {}", err),
    }
    // On the web (`wasm32-unknown-unknown`),
    // catching and unwinding panics is not possible.
    // It is better to avoid panicking code at all costs on the web.
    panic!("INTENTIONAL DEBUG PANIC");
  }

  /// A function that calls debug functions one by one.
  async fn call_debug_functions() -> Result<()> {
    Self::test_system_io().await?;
    Self::test_concurrency().await;
    Self::test_yielding().await;
    Self::test_parallelism().await;
    Ok(())
  }

  /// Tests async blocks.
  async fn test_concurrency() {
    let join_first = async {
      sleep(Duration::from_secs(1)).await;
      debug_print!("First future finished.");
    };
    let join_second = async {
      sleep(Duration::from_secs(2)).await;
      debug_print!("Second future finished.");
    };
    let join_third = async {
      sleep(Duration::from_secs(3)).await;
      debug_print!("Third future finished.");
    };
    join_first.await;
    join_second.await;
    join_third.await;
  }

  /// Avoids blocking the async event loop by yielding.
  async fn test_yielding() {
    let mut last_time = get_current_time();
    let mut count = 0u64;
    let mut steps_finished = 0;
    loop {
      for _ in 0..1000000 {
        count += 1;
      }
      yield_now().await;
      let time_passed = get_current_time() - last_time;
      if time_passed.num_milliseconds() > 1000 {
        debug_print!("Counted to {}, yielding regularly.", count);
        last_time = get_current_time();
        steps_finished += 1;
        if steps_finished == 10 {
          break;
        }
      }
    }
  }

  /// Tests functionalities that involve system calls.
  async fn test_system_io() -> Result<()> {
    sleep(Duration::from_secs(1)).await;
    debug_print!("Starting debug tests.");

    // Get the current time.
    let current_time = get_current_time();
    debug_print!("System time: {}", current_time);

    // Fetch data from a web API.
    let url = "http://jsonplaceholder.typicode.com/todos/1";
    let web_response = fetch_from_web_api(url).await?;
    debug_print!("Response from a web API: {:?}", web_response);

    // Use a crate that accesses operating system APIs.
    let hwid = get_hardward_id()?;
    debug_print!("Hardware ID: {:?}", hwid);

    Ok(())
  }

  /// Tests `spawn_blocking` with multicore parallelization.
  async fn test_parallelism() {
    let mut join_handles = Vec::new();
    let chunk_size = 10_i32.pow(6);
    for level in 0..10 {
      let join_handle = spawn_blocking(move || {
        let mut prime_count = 0;
        let count_from = level * chunk_size + 1;
        let count_to = (level + 1) * chunk_size;
        for number in count_from..=count_to {
          let mut is_prime = true;
          let square_root = (number as f64).sqrt() as i32;
          if number <= 1 {
            is_prime = false;
          } else {
            let mut i = 2;
            while i <= square_root {
              if number % i == 0 {
                is_prime = false;
                break;
              }
              i += 1;
            }
          }
          if is_prime {
            prime_count += 1;
          }
        }
        format!(
          "There are {prime_count} primes from {count_from} to {count_to}.",
        )
      });
      join_handles.push(join_handle);
    }
    for join_handle in join_handles {
      if let Ok(text) = join_handle.await {
        debug_print!("{}", text);
      }
    }
  }
}
