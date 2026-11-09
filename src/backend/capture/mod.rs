pub mod camera;
pub mod screen_cap;

pub use camera::VideoCapture;
pub use camera::CaptureStats;
pub use screen_cap::ScreenCapture;

use anyhow::{anyhow, Result};
use crate::backend::engine::frame::Frame;

pub trait Capture {
    /// Capture the next available frame.
    fn capture(&mut self) -> Result<Frame>;

    /// Return the capture dimensions.
    fn dimensions(&self) -> (u32, u32);
}

/// The `CaptureConfig` struct used to setup capture source
#[derive(Debug, Clone)]
pub struct CaptureConfig {
    pub device_id: u32,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            device_id: 0,
            width: 1920,
            height: 1080,
            fps: 30,
        }
    }
}

impl CaptureConfig {
    /// Check if the `CaptureConfig` setup is valid
    pub fn validate(&self) -> Result<()> {
        if self.width == 0 || self.height == 0 {
            return Err(anyhow!("Invalid camera resolution"));
        }

        if self.fps == 0 {
            return Err(anyhow!("FPS must be greater than zero"));
        }

        Ok(())
    }
}
