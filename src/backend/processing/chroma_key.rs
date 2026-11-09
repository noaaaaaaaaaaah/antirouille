use rustychroma;
use image::{RgbImage, RgbaImage};

/// `ChromaSettings` struct used to setup the `ChromaKey`
#[derive(Clone)]
pub struct ChromaSettings {
    pub key_color: [u8; 3],
    pub min_threshold: f64,
    pub max_threshold: f64,
}

impl Default for ChromaSettings {
    fn default() -> Self {
        Self {
            key_color: [0, 255, 0],
            min_threshold: 100.0,
            max_threshold: 7000.0,
        }
    }
}

/// The main `ChromaKey` struct, used to start chroma key processes
pub struct ChromaKey {
    settings: ChromaSettings,
}

impl ChromaKey {
    /// Create a new `ChromaKey` instance
    pub fn new(settings: ChromaSettings) -> Self {
        Self { settings }
    }

    /// Processes an image. Use the .Image in a `Frame` struct
    pub fn process(&self, input: &RgbImage) -> RgbaImage {
        // Convert RGB to RGBA.
        let mut output =
            image::DynamicImage::ImageRgb8(input.clone())
                .to_rgba8();

        // Apply rustychroma.
        rustychroma::remove_range(
            output.as_mut(),
            self.settings.key_color[0],
            self.settings.key_color[1],
            self.settings.key_color[2],
            self.settings.min_threshold,
            self.settings.max_threshold,
        );

        output
    }

    /// Update the `ChromaSettings` of the `ChromaKey` instance on the fly
    pub fn update_settings(&mut self, settings: ChromaSettings) {
        self.settings = settings;
    }
}