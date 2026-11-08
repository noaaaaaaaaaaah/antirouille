pub mod backend;

use anyhow::Result;

pub fn run() -> Result<()> {
    let mut camera = backend::VideoCapture::new(0);
    camera.start()?;

    let devices = backend::VideoCapture::list_devices()?;
    println!("Devices: {}", devices.len());

    let frame = camera.get_frame()?;

    println!(
        "Frame: {}x{}",
        frame.width(),
        frame.height()
    );
    
    camera.stop()?;

    Ok(())
}