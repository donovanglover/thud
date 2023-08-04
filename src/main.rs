use image::*;
use imageops::FilterType;

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("cover.png").unwrap();
    let new_img = DynamicImage::resize_to_fill(&img, 512, 512, FilterType::Nearest);

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", new_img.dimensions());

    // Write the contents of this image to the Writer in PNG format.
    new_img.save("test.png").unwrap();
}
