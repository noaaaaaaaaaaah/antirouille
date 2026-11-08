use anyhow::{Ok, Result, anyhow};
use image::RgbImage;
use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    query,
    utils::{ApiBackend, CameraIndex, CameraInfo, RequestedFormat, RequestedFormatType},
};

use std::time::Duration;

pub struct VideoCapture {
    stream: Option<Camera>,
    device_id: u32,
}

impl VideoCapture {
    ///Initialize Nokhwa
    pub fn init_nokhwa() {
        nokhwa::nokhwa_initialize(|x| {
            println!("Nokhwa Initalized: {x}");
        });
        std::thread::sleep(Duration::from_millis(2000));
    }

    ///Create a new Video Capture Device
    pub fn new(device_id: u32) -> Self {
        Self {
            stream: None,
            device_id,
        }
    }

    ///Start the Device
    pub fn start(&mut self) -> Result<()> {
        //Check if camera is already set
        if self.stream.is_some() {
            return Ok(());
        }
        //Setup the Device and start the stream
        let index = CameraIndex::Index(self.device_id);
        let request =
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut device = Camera::new(index, request)?;
        device.open_stream()?;
        self.stream = Some(device);
        Ok(())
    }

    ///Capture a frame
    pub fn get_frame(&mut self) -> Result<RgbImage> {
        let current_stream = self
            .stream
            .as_mut()
            .ok_or_else(|| anyhow!("Stream is not started"))?; // .ok_or_else avoids executing code if variable is Some
        //Capture the frame
        let frame = current_stream.frame()?;
        let image = frame.decode_image::<RgbFormat>()?;
        Ok(image)
    }

    ///Stop
    pub fn stop(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            stream.stop_stream()?;
        }

        Ok(())
    }

    /// List all available capture devices.
    pub fn list_devices() -> Result<Vec<CameraInfo>> {
        let devices = query(ApiBackend::Auto)?;
        Ok(devices)
    }

    /// Change the actual camera framerate.
    pub fn set_fps(&mut self, fps: u32) -> Result<u32> {
        let camera = self
            .stream
            .as_mut()
            .ok_or_else(|| anyhow!("Stream is not started"))?;
        camera.stop_stream()?;
        camera.set_frame_rate(fps)?;
        camera.open_stream()?;
        Ok(camera.frame_rate())
    }

    /// Get the actual camera framerate.
    pub fn get_fps(&self) -> Result<u32> {
        let camera = self
            .stream
            .as_ref()
            .ok_or_else(|| anyhow!("Stream is not started"))?;
        Ok(camera.frame_rate())
    }
}

impl Drop for VideoCapture {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
