use imagequant;
use wasm_bindgen::prelude::{wasm_bindgen, JsError};

#[wasm_bindgen]
pub struct ImagequantImage {
    pixels: Vec<imagequant::RGBA>,
    width: usize,
    height: usize,
    gamma: f64,
}

#[wasm_bindgen]
impl ImagequantImage {
    /// Make an image from RGBA pixels.
    /// Use 0.0 for gamma if the image is sRGB (most images are).
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>, width: usize, height: usize, gamma: f64) -> ImagequantImage {
        let pixels: Vec<imagequant::RGBA> = data
            .chunks(4)
            .map(|chunk| imagequant::RGBA {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            })
            .collect();

        ImagequantImage {
            pixels,
            width,
            height,
            gamma,
        }
    }
}

#[wasm_bindgen]
pub struct Imagequant {
    instance: imagequant::Attributes,
}

#[wasm_bindgen]
impl Imagequant {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Imagequant {
        Imagequant {
            instance: imagequant::new(),
        }
    }

    /// Make an image from RGBA pixels.
    /// Use 0.0 for gamma if the image is sRGB (most images are).
    pub fn new_image(data: Vec<u8>, width: usize, height: usize, gamma: f64) -> ImagequantImage {
        ImagequantImage::new(data, width, height, gamma)
    }

    /// It's better to use `set_quality()`
    pub fn set_max_colors(&mut self, max_colors: u32) -> Result<(), JsError> {
        self.instance
            .set_max_colors(max_colors)
            .map_err(JsError::from)
    }

    /// Range 0-100, roughly like JPEG.
    ///
    /// If the minimum quality can't be met, the quantization will be aborted with an error.
    ///
    /// Default is min 0, max 100, which means best effort, and never aborts the process.
    ///
    /// If max is less than 100, the library will try to use fewer colors.
    /// Images with fewer colors are not always smaller, due to increased dithering it causes.
    pub fn set_quality(&mut self, minimum: u8, target: u8) -> Result<(), JsError> {
        self.instance
            .set_quality(minimum, target)
            .map_err(JsError::from)
    }

    /// 1-10.
    ///
    /// Faster speeds generate images of lower quality, but may be useful
    /// for real-time generation of images.
    ///
    /// The default is 4.
    pub fn set_speed(&mut self, value: i32) -> Result<(), JsError> {
        self.instance.set_speed(value).map_err(JsError::from)
    }

    /// Number of least significant bits to ignore.
    ///
    /// Useful for generating palettes for VGA, 15-bit textures, or other retro platforms.
    pub fn set_min_posterization(&mut self, value: u8) -> Result<(), JsError> {
        self.instance
            .set_min_posterization(value)
            .map_err(JsError::from)
    }

    /// Create PNG based on specified settings
    pub fn process(&mut self, image: ImagequantImage) -> Result<Vec<u8>, JsError> {
        let ref mut liq_image = self
            .instance
            .new_image(image.pixels, image.width, image.height, image.gamma)
            .map_err(JsError::from)?;

        let mut res = self.instance.quantize(liq_image).map_err(JsError::from)?;

        let (palette, pixels) = res.remapped(liq_image).map_err(JsError::from)?;

        let mut encoder = lodepng::Encoder::new();
        encoder
            .set_palette(palette.as_slice())
            .map_err(JsError::from)?;

        let png_vec: Vec<u8> = encoder
            .encode(pixels.as_slice(), image.width, image.height)
            .map_err(JsError::from)?;

        Ok(png_vec)
    }
}
