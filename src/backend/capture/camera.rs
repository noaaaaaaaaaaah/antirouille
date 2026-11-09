
use anyhow::{anyhow, Context, Result};

use nokhwa::{
    pixel_format::RgbFormat,
    query,
    utils::{
        ApiBackend,
        CameraIndex,
        CameraInfo,
        RequestedFormat,
        RequestedFormatType,
        Resolution,
    },
    Camera,
};

use std::time::{Duration, Instant};

use crate::backend::engine::frame::{
    FrameSource,
    Frame,
};
use crate::backend::capture::{Capture, CaptureConfig};


///The `CaptureStats` struct used to store `VideoCapture` metadata
#[derive(Debug, Clone, Copy, Default)]
pub struct CaptureStats {
    pub frames_captured: u64,
    pub fps: f64,
    pub dropped_frames: u64,
}

///The main `VideoCapture` struct. It stores the nokhwa `Camera` and capture metadata
pub struct VideoCapture {
    camera: Option<Camera>,
    config: CaptureConfig,

    sequence: u64,
    started: bool,

    stats: CaptureStats,
    fps_counter: u64,
    fps_timer: Instant,
}

impl VideoCapture {
    /// Called to initialize nokhwa (required only for MacOS, see `nokhwa_initialize`)
    pub fn init_nokhwa() {
        nokhwa::nokhwa_initialize(|result| {
            println!("Nokhwa initialization: {result}");
        });
    }

    /// Create a new `VideoCapture` instance from a `CaptureConfig`
    pub fn new(config: CaptureConfig) -> Result<Self> {
        config.validate()?;

        Ok(Self {
            camera: None,
            config,
            sequence: 0,
            started: false,
            stats: CaptureStats::default(),
            fps_counter: 0,
            fps_timer: Instant::now(),
        })
    }

    /// `list_devices` function used to check available devices
    pub fn list_devices() -> Result<Vec<CameraInfo>> {
        query(ApiBackend::Auto)
            .context("Failed to enumerate cameras")
    }

    /// Open the stream of the `VideoCapture`
    pub fn start(&mut self) -> Result<()> {
        if self.started {
            return Ok(());
        }

        self.config.validate()?;

        let index = CameraIndex::Index(self.config.device_id);

        let requested_format = RequestedFormat::new::<RgbFormat>(
            RequestedFormatType::AbsoluteHighestFrameRate,
        );

        let mut camera = Camera::new(index, requested_format)
            .context("Failed to initialize camera")?;

        camera.set_resolution(
            Resolution::new(
                self.config.width,
                self.config.height,
            ),
        )
        .context("Failed to configure camera resolution")?;

        camera.set_frame_rate(self.config.fps)
            .context("Failed to configure camera FPS")?;

        camera.open_stream()
            .context("Failed to open camera stream")?;

        self.camera = Some(camera);
        self.started = true;

        self.sequence = 0;
        self.fps_counter = 0;
        self.fps_timer = Instant::now();

        Ok(())
    }

    /// Set the resolution of a `VideoCapture` struct
    pub fn set_resolution(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<()> {
        if width == 0 || height == 0 {
            return Err(anyhow!("Invalid resolution"));
        }

        let previous_width = self.config.width;
        let previous_height = self.config.height;
        let was_started = self.started;

        if was_started {
            self.stop()?;
        }

        self.config.width = width;
        self.config.height = height;

        if was_started {
            if let Err(error) = self.start() {
                self.config.width = previous_width;
                self.config.height = previous_height;

                if let Err(restore_error) = self.start() {
                    return Err(anyhow!(
                        "Resolution change failed: {error}. \
                         Camera restoration failed: {restore_error}"
                    ));
                }

                return Err(error);
            }
        }

        Ok(())
    }

    /// Set the fps of a `VideoCapture` struct
    pub fn set_fps(&mut self, fps: u32) -> Result<u32> {
        if fps == 0 {
            return Err(anyhow!("FPS must be greater than zero"));
        }

        let previous_fps = self.config.fps;
        let was_started = self.started;

        if was_started {
            self.stop()?;
        }

        self.config.fps = fps;

        if was_started {
            if let Err(error) = self.start() {
                self.config.fps = previous_fps;

                if let Err(restore_error) = self.start() {
                    return Err(anyhow!(
                        "FPS change failed: {error}. \
                         Camera restoration failed: {restore_error}"
                    ));
                }

                return Err(error);
            }
        }

        self.get_fps()
    }

    /// Read the current fps of a `VideoCapture` struct
    pub fn get_fps(&self) -> Result<u32> {
        match &self.camera {
            Some(camera) => Ok(camera.frame_rate()),
            None => Ok(self.config.fps),
        }
    }

    /// Read the current config of a `VideoCapture` struct
    pub fn get_config(&self) -> &CaptureConfig {
        &self.config
    }

    /// Handle the statistics of a `VideoCapture` struct
    pub fn get_stats(&self) -> CaptureStats {
        self.stats
    }

    pub fn reset_stats(&mut self) {
        self.stats = CaptureStats::default();
        self.fps_counter = 0;
        self.fps_timer = Instant::now();
    }

    /// Get the current `VideoCapture` status
    pub fn is_started(&self) -> bool {
        self.started && self.camera.is_some()
    }

    /// Stop the stream in the `VideoCapture`
    pub fn stop(&mut self) -> Result<()> {
        if let Some(mut camera) = self.camera.take() {
            self.started = false;

            camera.stop_stream()
                .context("Failed to stop camera stream")?;
        } else {
            self.started = false;
        }

        Ok(())
    }
}

/// Clean up for `VideoCapture`
impl Drop for VideoCapture {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

impl Capture for VideoCapture {
    fn capture(&mut self) -> Result<Frame>{
                if !self.started {
            return Err(anyhow!("Camera is not started"));
        }

        let camera = self.camera
            .as_mut()
            .ok_or_else(|| anyhow!("Camera unavailable"))?; //.or else avoids running programm when it's not needed

        let frame = camera.frame()
            .context("Failed to capture camera frame")?;

        let image = frame
            .decode_image::<RgbFormat>()
            .context("Failed to decode RGB frame")?;

        // Construct the shared engine frame.
        let video_frame = Frame::new(
            image,
            self.sequence,
            FrameSource::Webcam,
        );

        self.sequence += 1;

        // Update statistics.
        self.stats.frames_captured += 1;
        self.fps_counter += 1;

        let elapsed = self.fps_timer.elapsed();

        if elapsed >= Duration::from_secs(1) {
            self.stats.fps =
                self.fps_counter as f64 / elapsed.as_secs_f64();

            self.fps_counter = 0;
            self.fps_timer = Instant::now();
        }

        Ok(video_frame)
    }
    
    fn dimensions(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }
}