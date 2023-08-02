import { ImagequantImage, Imagequant } from "imagequant";

const inputElement = document.querySelector("input");

const beforeSizeElement = document.getElementById("before-size");
const afterSizeElement = document.getElementById("after-size");

const beforeImgElement = document.getElementById("before-img");
const afterImgElement = document.getElementById("after-img");

inputElement.addEventListener("change", async (event) => {
  // In reality this should probably be done in a Web Worker to not freeze the main thread

  const file = event.target.files[0];

  // Update UI
  beforeSizeElement.innerText = file.size + " bytes";
  beforeImgElement.src = URL.createObjectURL(file);

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

  const outputBlob = new Blob([output.buffer], { type: "image/png" });

  // Update UI
  afterSizeElement.innerText = outputBlob.size + " bytes";
  afterImgElement.src = URL.createObjectURL(outputBlob);
});
