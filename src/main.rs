extern crate image;

use image::GenericImage;

fn main() {
    // Use the open function to load an image from a Path.
    // ```open``` returns a `DynamicImage` on success.
    let img = image::open("test.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    img.save("test.png").unwrap();
}
