use anyhow::{Ok, Result, anyhow};
use nokhwa::{
    query,
    Camera,
    pixel_format::RgbFormat,
    utils::{
        ApiBackend,
        CameraInfo,
        CameraIndex,
        RequestedFormat,
        RequestedFormatType,
    },
};
use image::RgbImage;

pub struct VideoCapture {
    stream: Option<Camera>,
    device_id: u32,
}

impl VideoCapture {
    ///Create a new Video Capture Device
    pub fn new(device_id: u32) -> Self {
        Self {
            stream: None,
            device_id,
        }
    }

    ///Start the Device
    pub fn start(&mut self) -> Result<()>{
        //Check if camera is already set
        if self.stream.is_some() {
            return Ok(());
        }

        //Setup the Device and start the stream
        let index = CameraIndex::Index(self.device_id);
        let request = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate,);
        let mut device = Camera::new(index, request)?;

        device.open_stream()?;
        self.stream = Some(device);

        Ok(())
    }

    ///Capture a frame
    pub fn get_frame(&mut self) -> Result<RgbImage> {
        let current_stream = self.stream
            .as_mut()
            .ok_or_else(|| anyhow!("Stream is not started"))?; // .ok_or_else 

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
}

impl Drop for VideoCapture {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}