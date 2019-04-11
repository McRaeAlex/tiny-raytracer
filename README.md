# Tiny Raytacer
This is a implementation of a ray tracer in rust based off of ssloy's.
Each step will contain my code written for that step.

## Step 1:

This step just gets writing a image to work. I found the rust version to be much
more readable than the c++ version.

```rust
extern crate image;

fn render() {
    let width = 1024;
    let height = 768;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = ((x as f32 / width as f32) * 255.0) as u8;
        let green = ((y as f32 / height as f32) * 255.0) as u8;
        *pixel = image::Rgb([red, green, 0]);
    }

    imgbuf.save("oneframe.png").expect("Could not write image");
}

fn main() {
    render();
}
```