use std::time::{Duration, Instant};

pub struct GameLoop {
    target_fps: u64,
}

impl GameLoop {
    pub fn new(target_fps: u64) -> Self {
        Self { target_fps }
    }

    pub fn run<F>(&self, mut update_and_render: F)
    where
        F: FnMut(f32),
    {
        let target_frame_duration = Duration::from_secs_f64(1.0 / self.target_fps as f64);
        let mut last_time = Instant::now();

        loop {
            let now = Instant::now();
            let delta_time = (now - last_time).as_secs_f32();
            last_time = now;

            // Update and render logic
            update_and_render(delta_time);

            // Sleep to maintain the target frame rate
            let elapsed = now.elapsed();
            if elapsed < target_frame_duration {
                std::thread::sleep(target_frame_duration - elapsed);
            }
        }
    }
}