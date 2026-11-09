use image::RgbImage;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

/// Enumerate the possible source of a `VideoFrame`
pub enum FrameSource {
    Webcam,
    Screen,
    VideoFile,
    Network,
    Generated,
    StaticImage,
}

/// Enumerate the possible pixel format of a `VideoFrame` used for decoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb8,
    Rgba8,
    Bgr8,
    Nv12,
}

/// The `FrameMetadata` defines the metadata of a `VideoFrame`. 
/// It mainly contains both a sequence and instant value for playback management
#[derive(Debug, Clone, Copy)]
pub struct FrameMetadata {
    pub sequence: u64,
    pub timestamp: Instant,
    pub source: FrameSource,
}

/// The main `Frame` struct.
/// It's the type that is then handled by both processing and playback
/// Appart from the capture, the backend only handles `Frame`
#[derive(Debug, Clone)]
pub struct Frame {
    pub image: RgbImage,
    pub metadata: FrameMetadata,
}

impl Frame {
    pub fn new(
        image: RgbImage,
        sequence: u64,
        source: FrameSource,
    ) -> Self {
        Self {
            image,
            metadata: FrameMetadata {
                sequence,
                timestamp: Instant::now(),
                source,
            },
        }
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.image.dimensions()
    }
}