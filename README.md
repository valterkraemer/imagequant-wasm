# imagequant-wasm

[Imagequant](https://github.com/ImageOptim/libimagequant) bindings for JavaScript

Demo: [https://imagequant-wasm.pages.dev](https://imagequant-wasm.pages.dev)

## Install

```
npm i imagequant
```

## Using Vite

Need to install [`vite-plugin-wasm`](https://github.com/Menci/vite-plugin-wasm) and set it up according to it's instructions.

## Usage

```ts
import { ImagequantImage, Imagequant } from "imagequant";

const file = // A file

// Get pixels/imageData of image
const bitmap = await createImageBitmap(file);
const { width, height } = bitmap;

const canvas = new OffscreenCanvas(width, height);
const context = canvas.getContext("2d");
context.drawImage(bitmap, 0, 0, width, height);
const imageData = context.getImageData(0, 0, width, height);

// Need to send data as Uint8Array to Imagequant/WASM
const uint8Array = new Uint8Array(imageData.data.buffer);

const image = new ImagequantImage(uint8Array, width, height, 0);

const instance = new Imagequant();

// Apply options
instance.set_quality(0, 5);

// Do the work
const output = instance.process(image);

const blob = new Blob([output.buffer], { type: "image/png" });
```

See [demo/main.js](demo/main.js) for example usage.

## Options

May seem a bit verbose, but it tries to resemble the [Imagequant Rust documentation](https://docs.rs/imagequant/4.2.0/imagequant/index.html).

```ts
class ImagequantImage {
  constructor(data: Uint8Array, width: number, height: number, gamma: number);
}

class Imagequant {
  /**
   * Make an image from RGBA pixels.
   * Use 0.0 for gamma if the image is sRGB (most images are).
   */
  static new_image(
    data: Uint8Array,
    width: number,
    height: number,
    gamma: number
  ): ImagequantImage;

  /**
   * It's better to use `set_quality()`
   */
  set_max_colors(max_colors: number): void;

  /**
   * Range 0-100, roughly like JPEG.
   *
   * If the minimum quality can't be met, the quantization will be aborted with an error.
   *
   * Default is min 0, max 100, which means best effort, and never aborts the process.
   *
   * If max is less than 100, the library will try to use fewer colors.
   * Images with fewer colors are not always smaller, due to increased dithering it causes.
   */
  set_quality(minimum: number, target: number): void;

  /**
   * 1-10.
   *
   * Faster speeds generate images of lower quality, but may be useful
   * for real-time generation of images.
   *
   * The default is 4.
   */
  set_speed(value: number): void;

  /**
   * Number of least significant bits to ignore.
   *
   * Useful for generating palettes for VGA, 15-bit textures, or other retro platforms.
   */
  set_min_posterization(value: number): void;

  /**
   * Create PNG based on specified settings
   */
  process(image: ImagequantImage): Uint8Array;
}
```

## Build

```
wasm-pack build --target bundler
```
