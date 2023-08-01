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

    pub fn new_image(data: Vec<u8>, width: usize, height: usize, gamma: f64) -> ImagequantImage {
        ImagequantImage::new(data, width, height, gamma)
    }

    pub fn set_max_colors(&mut self, max_colors: u32) -> Result<(), JsError> {
        self.instance
            .set_max_colors(max_colors)
            .map_err(JsError::from)
    }

    pub fn set_quality(&mut self, minimum: u8, target: u8) -> Result<(), JsError> {
        self.instance
            .set_quality(minimum, target)
            .map_err(JsError::from)
    }

    pub fn set_speed(&mut self, value: i32) -> Result<(), JsError> {
        self.instance.set_speed(value).map_err(JsError::from)
    }

    pub fn set_min_posterization(&mut self, value: u8) -> Result<(), JsError> {
        self.instance
            .set_min_posterization(value)
            .map_err(JsError::from)
    }

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
