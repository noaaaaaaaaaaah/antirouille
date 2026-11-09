use xcap::{Monitor, VideoRecorder};
use image::{RgbImage, RgbaImage, DynamicImage};

use anyhow::{anyhow, Context, Result};
use std::iter;
use std::sync::mpsc::Receiver;

use crate::backend::engine::frame::{
    FrameSource,
    Frame,
};
use crate::backend::capture::{Capture, CaptureConfig};


/// The main `ScreenCapture`
pub struct ScreenCapture {
    recorder: VideoRecorder,
    receiver: Receiver<xcap::Frame>,
    config: CaptureConfig,

    sequence: u64,
}

impl ScreenCapture {
    /// Create a new `ScreenCapture` instance
    pub fn new(config: CaptureConfig) -> Result<Self> {
        config.validate()?;
        
        let monitor = Monitor::all()?
            .into_iter()
            .find(|m|m.id().ok() == Some(config.device_id))
            .expect("Monitor ID not found");

        let (video_recorder, sx) = monitor.video_recorder().unwrap();

        Ok(Self { 
            recorder: video_recorder,
            receiver: sx,
            config,
            sequence: 0 
        })
    }

    /// `list_monitors` function used to check available monitors
    pub fn list_monitors() -> Result<Vec<Monitor>> {
        Ok(Monitor::all().unwrap())
    }
}

/// Clean up for `ScreenCapture`
impl Drop for ScreenCapture {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Capture implementation specific to `ScreenCapture`
impl Capture for ScreenCapture {
    fn capture(&mut self) -> Result<Frame> {
        todo!()
    }

    fn dimensions(&self) -> (u32, u32) {
        todo!()
    }
}