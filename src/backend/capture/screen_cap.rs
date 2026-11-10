use xcap::{Monitor, VideoRecorder};
use image::{DynamicImage, RgbImage, RgbaImage};

use anyhow::{Context, Ok, Result, anyhow};
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

    started: bool,
    sequence: u64,
}

impl ScreenCapture {
    /// Create a new `ScreenCapture` instance
    /// The `CaptureConfig.device_id` select monitor by it's id
    pub fn new(config: CaptureConfig) -> Result<Self> {
        config.validate()?;
        
        let mut monitor_config = config;

        let monitor = Monitor::all()?
            .into_iter()
            .find(|m| m.id().ok() == Some(monitor_config.device_id))
            .context("Monitor ID not found")?;

        monitor_config.width = monitor.width()?;
        monitor_config.height = monitor.height()?;

        let (video_recorder, sx) = monitor.video_recorder().unwrap();

        Ok(Self { 
            recorder: video_recorder,
            receiver: sx,
            config: monitor_config,
            started: false,
            sequence: 0 
        })
    }
    
    /// `convert_raw_to_rgb` is a utils that converts 
    /// raw pixel data found in `xcap::Frame` to RGB
    /// to then use it in a `Frame`
    fn convert_raw_to_rgb(frame: xcap::Frame) -> Result<RgbImage> {
        let converted_frame = RgbaImage::from_raw(
            frame.width, 
            frame.height, 
            frame.raw
        ).context("Couldn't create RGB image from the xcap::Frame raw data")?;
        Ok(DynamicImage::ImageRgba8(converted_frame).to_rgb8())
        
    }

    /// Start the `ScreenCapture` instance
    pub fn start(&mut self) -> Result<()> {
        if self.started {
            return Ok(());
        }

        self.config.validate()?;

        self.recorder.start()?;
        self.started = true;

        Ok(())
    }
    
    /// Stop the `ScreenCapture` instance
    pub fn stop(&mut self) -> Result<()> {
        if !self.started {
            return Ok(());
        }

        self.recorder.stop()?;
        self.started = false;

        Ok(())
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
        if !self.started {
            return Err(anyhow!("Capture is not started"));
        }

        let raw_frame = self.receiver.recv()
            .context("Failed to receive frame")?;

        let image = Self::convert_raw_to_rgb(raw_frame)?;
        let frame = Frame::new(
            image, 
            self.sequence, 
            FrameSource::Screen
        );

        self.sequence += 1;

        Ok(frame)
    }

    fn dimensions(&self) -> (u32, u32) {
        let dimensions = (
            self.config.width, 
            self.config.height
        );
        return dimensions
    }
}