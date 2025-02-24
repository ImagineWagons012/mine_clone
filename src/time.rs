use std::time::{Instant, Duration};


pub struct Time {
    frame_start_time: Instant,
    update_start_time: Instant,
    render_start_time: Instant,
    render_duration: Duration,
    update_duration: Duration,
    frame_time: Duration,
}

impl Time {
    pub fn new() -> Self {
        Self { 
            frame_start_time: Instant::now(), 
            update_start_time: Instant::now(), 
            render_start_time: Instant::now(), 
            render_duration: Duration::from_millis(10), 
            update_duration: Duration::from_millis(10), 
            frame_time: Duration::from_millis(10),
        }
    }
    pub fn update_frame_time(&mut self) {
        self.frame_time = self.frame_start_time.elapsed();
    }
    pub fn update_render_time(&mut self) {
        self.render_duration = self.render_start_time.elapsed();
    }
    pub fn gpu_time(&self) -> f32 {
        self.render_duration.as_secs_f32()
    }
    pub fn update_update_time(&mut self) {
        self.update_duration = self.render_start_time.elapsed();
    }
    pub fn cpu_time(&self) -> f32 {
        self.update_duration.as_secs_f32()
    }
    // elapsed time since last update started in seconds (f32)
    pub fn delta_time(&self) -> f32 {
        self.frame_time.as_secs_f32()
    }
    pub fn set_frame_start_time(&mut self) {
        self.frame_start_time = Instant::now()
    }
    pub fn set_update_start_time(&mut self) {
        self.update_start_time = Instant::now()
    }
    pub fn set_render_start_time(&mut self) {
        self.render_start_time = Instant::now()
    }
}