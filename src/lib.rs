pub mod backend;

use anyhow::Result;

use backend::capture::{VideoCapture, ScreenCapture, CaptureConfig};
use backend::processing::{ChromaKey, ChromaSettings};

use std::fs;

use crate::backend::capture::Capture;

pub fn run() -> Result<()> {
    VideoCapture::init_nokhwa();
    
    let mut camera = VideoCapture::new(
        CaptureConfig::default()
    )?;

    camera.start()?;
    let captured_frame = camera.capture()?;
    let settings = ChromaSettings { 
        key_color: [0, 0, 190], 
        min_threshold: 100.0, 
        max_threshold: 10000.0
    };
    let chroma_process = ChromaKey::new(settings);
    let processed = chroma_process.process(&captured_frame.image);


    fs::create_dir_all("rec")?;
    processed.save("rec/chroma_keyed.png")?;

    camera.stop()?;
    
    Ok(())
}
