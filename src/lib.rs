pub mod backend;

use anyhow::{Context, Result};

use backend::capture::{VideoCapture, ScreenCapture,  CaptureConfig};
use backend::processing::{ChromaKey, ChromaSettings};
use xcap::Monitor;

use std::fs;

use crate::backend::capture::Capture;

pub fn run() -> Result<()> {
    // VIDEO CAPTURE TEST
    VideoCapture::init_nokhwa();
    
    let mut camera = VideoCapture::new(
        CaptureConfig::default()
    )?;

    camera.start()?;
    let captured_frame = camera.capture()?;
    let settings = ChromaSettings { 
        key_color: [0, 0, 190], 
        min_threshold: 100.0, 
        max_threshold: 7000.0
    };
    let chroma_process = ChromaKey::new(settings);
    let processed = chroma_process.process(&captured_frame.image);

    fs::create_dir_all("rec")?;
    processed.save("rec/chroma_keyed.png")?;

    camera.stop()?;

    // SCREEN CAPTURE TEST
    let monitors = Monitor::all()?;
    let monitor = monitors
        .first()
        .context("No monitor found")?;

    let mut screen = ScreenCapture::new(
        CaptureConfig { 
            device_id: monitor.id()?, 
            width: monitor.width()?, 
            height: monitor.height()?, 
            fps: 30
        }
    )?;

    screen.start()?;
    screen.capture()?;
    let captured_screen = screen.capture()?;
    let chroma_cap = chroma_process.process(&captured_screen.image);
    chroma_cap.save("rec/chroma_cap.png")?;
    screen.stop()?;
    
    Ok(())
}
