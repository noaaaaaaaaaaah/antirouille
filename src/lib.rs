pub mod backend;

use anyhow::Result;

use backend::{VideoCapture, ChromaKey, ChromaSettings};

use std::fs;

pub fn run() -> Result<()> {
    VideoCapture::init_nokhwa();
    let mut camera = backend::VideoCapture::new(0);
    camera.start()?;
    let capture_frame = camera.get_frame()?;

    let chroma_settings = backend::ChromaSettings {
        key_color: [0, 0, 123],
        min_threshold: 100.0,
        max_threshold: 7000.0,
    };
    let chroma_process = backend::ChromaKey::new(chroma_settings);
    let processed = chroma_process.process(&capture_frame);

    fs::create_dir_all("output")?;
    processed.save("output/chroma_keyed.png")?;

    camera.stop()?;

    Ok(())
}
